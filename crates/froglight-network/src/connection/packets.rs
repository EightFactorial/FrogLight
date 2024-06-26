use std::{
    collections::VecDeque,
    io::{Cursor, Read},
};

use async_compression::futures::{bufread::ZlibDecoder, write::ZlibEncoder};
use async_std::{
    io::{BufReader, ReadExt, WriteExt},
    net::TcpStream,
};
use froglight_protocol::{
    protocol::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite, ReadError, WriteError},
    traits::{State, Version},
};
use futures_lite::AsyncBufReadExt;

use super::{Connection, ConnectionError, NetworkDirection, ReadConnection, WriteConnection};

impl<V, S, D> Connection<V, S, D>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    /// Sends a packet through the connection.
    ///
    /// # Errors
    /// If a packet cannot be sent.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn send(&mut self, packet: impl Into<D::Send>) -> Result<(), ConnectionError> {
        self.send_packet(&packet.into()).await
    }

    /// Sends a packet through the connection.
    ///
    /// # Errors
    /// If a packet cannot be sent.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn send_packet(&mut self, packet: &D::Send) -> Result<(), ConnectionError> {
        send_packet::<V, S, D>(self.stream.get_mut(), &self.compression, packet).await
    }

    /// Receive a packet from the other side of the connection.
    ///
    /// # Errors
    /// If a packet cannot be received.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn recv(&mut self) -> Result<D::Recv, ConnectionError> {
        recv_packet::<V, S, D>(&mut self.stream, &mut self.bundle, &self.compression).await
    }
}

impl<V, S, D> ReadConnection<V, S, D>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    /// Receive a packet from the other side of the connection.
    ///
    /// # Errors
    /// If a packet cannot be received.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn recv(&mut self) -> Result<D::Recv, ConnectionError> {
        recv_packet::<V, S, D>(&mut self.stream, &mut self.bundle, &*self.compression.read().await)
            .await
    }
}

impl<V, S, D> WriteConnection<V, S, D>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    /// Sends a packet through the connection.
    ///
    /// # Errors
    /// If a packet cannot be sent.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn send(&mut self, packet: impl Into<D::Send>) -> Result<(), ConnectionError> {
        self.send_packet(&packet.into()).await
    }

    /// Sends a packet through the connection.
    ///
    /// # Errors
    /// If a packet cannot be sent.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn send_packet(&mut self, packet: &D::Send) -> Result<(), ConnectionError> {
        send_packet::<V, S, D>(&mut self.stream, &*self.compression.read().await, packet).await
    }
}

/// Sends a packet through the connection.
///
/// # Errors
/// If a packet cannot be sent.
///
/// # Panics
/// If the packet length overflows.
async fn send_packet<V, S, D>(
    stream: &mut TcpStream,
    compression: &Option<i32>,
    packet: &D::Send,
) -> Result<(), ConnectionError>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    // Create a buffer to store the packet bytes
    let mut buffer: Vec<u8> = packet.fg_to_bytes();

    if let Some(threshold) = compression {
        if i32::try_from(buffer.len()).expect("Packet length overflow") >= *threshold {
            // Compress the buffer, prefixing it with the uncompressed length
            let mut compressor = ZlibEncoder::new(buffer.len().fg_var_to_bytes());
            compressor.write_all(&buffer).await?;

            buffer = compressor.into_inner();
        } else {
            // Prefix the buffer with `0` to indicate that it is not compressed
            let mut prefixed_buffer = Vec::with_capacity(buffer.len() + 1);
            0u32.fg_var_write(&mut prefixed_buffer)?;
            prefixed_buffer.append(&mut buffer);

            buffer = prefixed_buffer;
        }
    }

    // Prefix the buffer with its length
    prefix_length(&mut buffer)?;

    // TODO: Encryption

    // Write the buffer to the stream
    stream.write_all(&buffer).await.map_err(ConnectionError::from)
}

/// Receive a packet from the other side of the connection.
///
/// # Errors
/// If a packet cannot be received.
///
/// # Panics
/// If the packet length overflows.
async fn recv_packet<V, S, D>(
    buffer: &mut BufReader<TcpStream>,
    bundle: &mut VecDeque<D::Recv>,
    compression: &Option<i32>,
) -> Result<D::Recv, ConnectionError>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    // If there are any bundled packets, return the first one
    if let Some(packet) = bundle.pop_front() {
        return Ok(packet);
    }

    // Get the byte buffer
    let buffer_bytes = buffer.fill_buf().await?;
    let buffer_len = buffer_bytes.len();

    // If the buffer is empty, the connection is closed
    if buffer_len == 0 {
        return Err(ConnectionError::ConnectionClosed);
    }

    // TODO: Decryption

    // Read the packet length from the buffer
    let mut cursor = Cursor::new(buffer_bytes);

    let packet_length = usize::fg_var_read(&mut cursor)?;
    let packet_length_bytes = usize::try_from(cursor.position()).expect("Packet length overflow");

    // Read the packet from the buffer
    let mut packet_buffer = vec![0u8; packet_length];
    cursor.read_exact(&mut packet_buffer)?;
    buffer.consume(packet_length_bytes + packet_length);

    let mut cursor = Cursor::new(packet_buffer.as_slice());

    // If the packet is compressed, decompress it
    if compression.is_some_and(|c| c >= 0) && 0 != u32::fg_var_read(&mut cursor)? {
        let current_position = usize::try_from(cursor.position()).expect("Packet length overflow");
        let slice = &cursor.get_ref()[current_position..];

        // Decompress the packet
        let mut decompressor = ZlibDecoder::new(slice);
        let mut decompressed = Vec::with_capacity(packet_length);
        decompressor.read_to_end(&mut decompressed).await?;

        // Get the length of the decompressed packet
        let decompressed_packet_length = decompressed.len();

        // Read the packet from the decompressed buffer
        let mut cursor = Cursor::new(decompressed.as_slice());
        let packet = D::Recv::fg_read(&mut cursor);

        // Read any bundled packets from the decompressed buffer
        if packet.is_ok() && cursor.position() == 0u64 {
            #[allow(unused_variables)]
            if let Err(err) =
                read_bundled::<V, S, D>(decompressed_packet_length, &mut cursor, bundle)
            {
                #[cfg(feature = "bevy")]
                bevy_log::error!("Error reading bundled packets: {err}");
            }
        }

        // Return the packet
        packet.map_err(ConnectionError::from)
    } else {
        // Read the packet from the buffer
        let packet = D::Recv::fg_read(&mut cursor);

        // Read any bundled packets from the buffer
        if packet.is_ok() && cursor.position() == 0u64 {
            #[allow(unused_variables)]
            if let Err(err) = read_bundled::<V, S, D>(packet_length, &mut cursor, bundle) {
                #[cfg(feature = "bevy")]
                bevy_log::error!("Error reading bundled packets: {err}");
            }
        }

        // Return the packet
        packet.map_err(ConnectionError::from)
    }
}

/// Prefixes the given buffer with its length.
fn prefix_length(buf: &mut Vec<u8>) -> Result<(), WriteError> {
    let mut prefixed_buffer = Vec::with_capacity(buf.len() + 5);
    buf.len().fg_var_write(&mut prefixed_buffer)?;
    prefixed_buffer.append(buf);
    *buf = prefixed_buffer;
    Ok(())
}

fn read_bundled<V, S, D>(
    packet_length: usize,
    cursor: &mut Cursor<&[u8]>,
    bundle: &mut VecDeque<D::Recv>,
) -> Result<(), ReadError>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    while packet_length > usize::try_from(cursor.position()).expect("Packet length overflow") {
        match D::Recv::fg_read(cursor) {
            Ok(packet) => bundle.push_back(packet),
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
