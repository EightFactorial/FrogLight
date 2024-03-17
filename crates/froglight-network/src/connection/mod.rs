use std::{collections::VecDeque, io::Cursor, marker::PhantomData, net::SocketAddr};

use async_compression::futures::{bufread::ZlibDecoder, write::ZlibEncoder};
use async_std::{
    io::{BufReader, ReadExt, WriteExt},
    net::TcpStream,
};
use bevy_tasks::futures_lite::AsyncBufReadExt;
use froglight_protocol::{
    io::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite, ReadError, WriteError},
    states::Handshaking,
    traits::{State, Version},
};

mod conversion;

mod direction;
pub use direction::{Clientbound, NetworkDirection, Serverbound};

mod error;
pub use error::ConnectionError;

use crate::resolver::Resolver;

/// A connection to a server or client.
#[derive(Debug)]
pub struct Connection<V: Version, S: State<V>, D: NetworkDirection<V, S> = Serverbound> {
    pub(crate) stream: TcpStream,
    pub(crate) buffer: BufReader<TcpStream>,
    pub(crate) bundle: VecDeque<D::Recv>,
    pub(crate) compression: Option<i32>,
    _version: PhantomData<V>,
    _state: PhantomData<S>,
    _direction: PhantomData<D>,
}

impl<V: Version, D: NetworkDirection<V, Handshaking>> Connection<V, Handshaking, D>
where
    Handshaking: State<V>,
{
    /// Connect to a server at an address resolved by the given resolver.
    ///
    /// # Errors
    /// If the address cannot be resolved.
    /// If the connection cannot be established.
    /// If the connection cannot be set to nodelay.
    pub async fn connect_to(
        address: &(impl AsRef<str> + ?Sized),
        resolver: &Resolver,
    ) -> Result<Self, ConnectionError> {
        let address = resolver.url_lookup(address.as_ref()).await?;
        Self::connect(address).await
    }

    /// Connect to a server at the given address.
    ///
    /// # Errors
    /// If the connection cannot be established.
    /// If the connection cannot be set to nodelay.
    #[inline]
    pub async fn connect(socket: SocketAddr) -> Result<Self, ConnectionError> {
        let stream = TcpStream::connect(socket).await?;
        Self::from_stream(stream).await
    }
}

#[allow(clippy::unused_async)]
impl<V: Version, S: State<V>, D: NetworkDirection<V, S>> Connection<V, S, D> {
    /// Send a packet to the other side of the connection.
    ///
    /// # Errors
    /// If a packet cannot be sent.
    ///
    /// # Panics
    /// If the packet length overflows.
    pub async fn send(&mut self, packet: impl Into<D::Send>) -> Result<(), ConnectionError> {
        // Create a buffer to store the packet bytes
        let mut buffer: Vec<u8> = packet.into().fg_to_bytes();

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
            return Err(ConnectionError::ConnecionClosed);
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

            // Read the packet from the decompressed buffer
            let mut cursor = Cursor::new(decompressed.as_slice());
            let packet = D::Recv::fg_read(&mut cursor)?;

            // Read any bundled packets from the decompressed buffer
            if cursor.position() < 1 {
                Self::read_bundled(packet_length, &mut cursor, &mut self.bundle)?;
            }

            // Consume the packet bytes from the original buffer
            self.buffer.consume(packet_length_bytes + packet_length);

            // Return the packet
            Ok(packet)
        } else {
            // Read the packet from the buffer
            let packet = D::Recv::fg_read(&mut cursor)?;

            // Read any bundled packets from the buffer
            if cursor.position() < 1 {
                Self::read_bundled(packet_length, &mut cursor, &mut self.bundle)?;
            }

            // Consume the packet bytes from the buffer
            self.buffer.consume(packet_length_bytes + packet_length);

            // Return the packet
            Ok(packet)
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

    /// Create a new connection from a TCP stream.
    ///
    /// # Errors
    /// The stream cannot get or set nodelay.
    pub async fn from_stream(stream: TcpStream) -> Result<Self, ConnectionError> {
        // Set the stream to nodelay
        if !stream.nodelay()? {
            stream.set_nodelay(true)?;
        }

        // Create a buffer to read from the stream
        let buffer = BufReader::new(stream.clone());

        // Create a buffer to store received bundled packets
        let bundle = VecDeque::with_capacity(8);

        Ok(Self {
            stream,
            buffer,
            bundle,
            compression: None,
            _version: PhantomData,
            _state: PhantomData,
            _direction: PhantomData,
        })
    }
}
