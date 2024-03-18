use std::{collections::VecDeque, io::Cursor};

use async_compression::futures::{bufread::ZlibDecoder, write::ZlibEncoder};
use async_std::io::{ReadExt, WriteExt};
use bevy_log::error;
use bevy_tasks::futures_lite::AsyncBufReadExt;
use froglight_protocol::{
    io::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite, ReadError, WriteError},
    traits::{State, Version},
};

use crate::{Connection, ConnectionError, NetworkDirection};

impl<V: Version, S: State<V>, D: NetworkDirection<V, S>> Connection<V, S, D> {
    /// Send a packet to the other side of the connection.
    ///
    /// # Errors
    /// If a packet cannot be sent.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn send(&mut self, packet: impl Into<D::Send>) -> Result<(), ConnectionError> {
        self.send_packet(packet.into()).await
    }

    /// Send a packet to the other side of the connection.
    ///
    /// # Errors
    /// If a packet cannot be sent.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn send_packet(&mut self, packet: D::Send) -> Result<(), ConnectionError> {
        // Create a buffer to store the packet bytes
        let mut buffer: Vec<u8> = packet.fg_to_bytes();

        if let Some(threshold) = self.compression {
            if i32::try_from(buffer.len()).expect("Packet length overflow") >= threshold {
                // Compress the buffer, prefixing it with `0` to indicate that it is compressed
                let mut compressor = ZlibEncoder::new(vec![0u8]);
                compressor.write_all(&buffer).await?;
                buffer = compressor.into_inner();
            } else {
                // Prefix the buffer with its uncompressed length
                Self::prefix_length(&mut buffer)?;
            }
        }

        // Prefix the buffer with its length
        Self::prefix_length(&mut buffer)?;

        // TODO: Encryption

        // Write the buffer to the stream
        Ok(self.stream.write_all(&buffer).await?)
    }

    /// Prefixes the given buffer with its length.
    fn prefix_length(buf: &mut Vec<u8>) -> Result<(), WriteError> {
        let mut prefixed_buffer = Vec::with_capacity(buf.len() + 5);
        buf.len().fg_var_write(&mut prefixed_buffer)?;
        prefixed_buffer.append(buf);
        *buf = prefixed_buffer;
        Ok(())
    }

    /// Receive a packet from the other side of the connection.
    ///
    /// # Errors
    /// If a packet cannot be received.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn recv(&mut self) -> Result<D::Recv, ConnectionError> {
        // If there are any bundled packets, return the first one
        if let Some(packet) = self.bundle.pop_front() {
            return Ok(packet);
        }

        // Get the byte buffer
        let buffer = self.buffer.fill_buf().await?;
        let buffer_len = buffer.len();

        // If the buffer is empty, the connection is closed
        if buffer_len == 0 {
            return Err(ConnectionError::ConnectionClosed);
        }

        // TODO: Decryption

        // Read the packet length from the buffer
        let mut cursor = Cursor::new(buffer);
        let packet_length = usize::fg_var_read(&mut cursor)?;

        // Consume the packet bytes from the buffer
        let packet_length_bytes =
            usize::try_from(cursor.position()).expect("Packet length overflow");

        // If the packet is compressed, decompress it
        if self.compression.is_some_and(|c| c >= 0) && 0 != u32::fg_var_read(&mut cursor)? {
            // Decompress the packet
            let mut decompressor = ZlibDecoder::new(
                &cursor.get_ref()[packet_length_bytes..packet_length_bytes + packet_length],
            );
            let mut decompressed = Vec::with_capacity(packet_length);
            decompressor.read_to_end(&mut decompressed).await?;

            // Get the length of the decompressed packet
            let decompressed_packet_length = decompressed.len();

            // Read the packet from the decompressed buffer
            let mut cursor = Cursor::new(decompressed.as_slice());
            let packet = D::Recv::fg_read(&mut cursor);

            // Read any bundled packets from the decompressed buffer
            if packet.is_ok() && cursor.position() == 0u64 {
                if let Err(err) =
                    Self::read_bundled(decompressed_packet_length, &mut cursor, &mut self.bundle)
                {
                    error!("Error reading bundled packets: {err}");
                }
            }

            // Consume the packet bytes from the original buffer
            self.buffer.consume(packet_length_bytes + packet_length);

            // Return the packet
            packet.map_err(ConnectionError::from)
        } else {
            // Read the packet from the buffer
            let packet = D::Recv::fg_read(&mut cursor);

            // Read any bundled packets from the buffer
            if packet.is_ok() && cursor.position() == 0u64 {
                if let Err(err) = Self::read_bundled(packet_length, &mut cursor, &mut self.bundle) {
                    error!("Error reading bundled packets: {err}");
                }
            }

            // Consume the packet bytes from the buffer
            self.buffer.consume(packet_length_bytes + packet_length);

            // Return the packet
            packet.map_err(ConnectionError::from)
        }
    }

    /// Attempt to read bundled packets from the given cursor.
    fn read_bundled(
        packet_length: usize,
        cursor: &mut Cursor<&[u8]>,
        bundle: &mut VecDeque<D::Recv>,
    ) -> Result<(), ReadError> {
        while packet_length > usize::try_from(cursor.position()).expect("Packet length overflow") {
            match D::Recv::fg_read(cursor) {
                Ok(packet) => bundle.push_back(packet),
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }
}
