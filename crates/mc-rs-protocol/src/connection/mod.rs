#![allow(unused_variables)]
#![allow(dead_code)]

use std::{collections::VecDeque, io::Cursor, marker::PhantomData};

use async_compression::futures::{bufread::ZlibDecoder, write::ZlibEncoder};
use async_net::{AsyncToSocketAddrs, SocketAddr, TcpStream};
use azalea_chat::FormattedText;
use compact_str::CompactString;
use futures_lite::{io::BufReader, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use thiserror::Error;

use crate::{
    buffer::{Decode, DecodeError, EncodeError, FromValue, VarDecode, VarEncode},
    versions::state::Handshake,
    State, Version,
};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Connection<V: Version, S: State<V>> {
    _version: PhantomData<V>,
    _state: PhantomData<S>,
    pub hostname: CompactString,
    pub port: u16,
    pub compression: Option<i32>,
    packet_buffer: VecDeque<<S as State<V>>::Clientbound>,
    buffer: BufReader<TcpStream>,
    stream: TcpStream,
}

impl<V: Version> Connection<V, Handshake>
where
    Handshake: State<V>,
{
    /// Create a new connection from an address.
    pub async fn new(
        version: V,
        address: impl Into<CompactString>,
    ) -> Result<Self, ConnectionError> {
        let address = address.into();
        let mut address = address.as_str();
        if let Some(pos) = address.find("://") {
            address = &address[pos + 3..];
        }

        if let Some(colon) = address.find(':') {
            let (address, port) = address.split_at(colon);
            let port: u16 = port[1..].parse()?;

            Self::new_from(address, port).await
        } else {
            Self::new_from(address, 25565).await
        }
    }
}

impl<V: Version, S: State<V>> Connection<V, S> {
    const BUFFER_SIZE: usize = 65536;

    /// Create a new connection from a hostname and port.
    pub async fn new_from(hostname: &str, port: u16) -> Result<Self, ConnectionError> {
        let stream = TcpStream::connect((hostname, port)).await?;

        Ok(Self {
            _version: PhantomData,
            _state: PhantomData,
            hostname: hostname.into(),
            port,
            compression: None,
            packet_buffer: VecDeque::new(),
            buffer: BufReader::with_capacity(Self::BUFFER_SIZE, stream.clone()),
            stream,
        })
    }

    /// Create a new connection from anything that can be converted to a socket address.
    pub async fn from(address: impl AsyncToSocketAddrs) -> Result<Self, ConnectionError> {
        let mut addresses = address.to_socket_addrs().await?;
        let address = addresses.next().ok_or(ConnectionError::NoAddressFound)?;
        Self::from_sock(address).await
    }

    /// Create a new connection from a socket address.
    pub async fn from_sock(sock: SocketAddr) -> Result<Self, ConnectionError> {
        let stream = TcpStream::connect(sock).await?;
        Ok(stream.try_into()?)
    }

    /// Sends a packet to the server.
    pub async fn send_packet(
        &mut self,
        packet: impl Into<<S as State<V>>::Serverbound>,
    ) -> Result<(), ConnectionError> {
        let packet = packet.into();

        #[cfg(feature = "debug")]
        {
            let mut string = format!("{packet:?}");
            if string.len() > 100 {
                string.truncate(97);
                string.push_str("...");
            }

            log::debug!("Sending packet: {string}");
        }

        let mut buf = Vec::from_value(&packet)?;

        // Compression
        if let Some(threshold) = self.compression {
            if threshold > buf.len() as i32 {
                // Write 0 to indicate compressed, then write the compressed data
                let mut compressor = ZlibEncoder::new(vec![0u8]);
                compressor.write_all(&buf).await?;
                buf = compressor.into_inner();
            } else {
                // Add the length of uncompressed data
                Self::add_length(&mut buf)?;
            }
        }

        // Add the length of the packet
        Self::add_length(&mut buf)?;

        // TODO: Encryption

        // Write the packet
        self.stream.write_all(&buf).await?;
        Ok(())
    }

    /// Add the length of the buffer to the front of the buffer.
    fn add_length(buf: &mut Vec<u8>) -> Result<(), ConnectionError> {
        let mut len_buf = Vec::with_capacity(buf.len() + 2);

        buf.len().var_encode(&mut len_buf)?;
        len_buf.extend_from_slice(buf);

        *buf = len_buf;
        Ok(())
    }

    /// Receives a packet from the server.
    pub async fn receive_packet(
        &mut self,
    ) -> Result<<S as State<V>>::Clientbound, ConnectionError> {
        // Return a packet from the buffer if possible
        if let Some(packet) = self.packet_buffer.pop_front() {
            #[cfg(feature = "debug")]
            log::trace!("Packet buffer len: {}", self.packet_buffer.len());

            return Ok(packet);
        }

        // Read the packet from the stream
        let buffer = self.buffer.fill_buf().await?;
        let buffer_len = buffer.len();

        if buffer_len == 0 {
            return Err(ConnectionError::Closed);
        }

        #[cfg(feature = "debug")]
        log::trace!(
            "Byte peek: {:?}",
            &buffer[0..std::cmp::min(16, buffer.len())]
        );

        // TODO: Decryption

        // Read the length of the packet
        let mut cursor = Cursor::new(buffer);
        let packet_len = u32::var_decode(&mut cursor)?;

        // Consume the length bytes
        {
            let len_byte_count = cursor.position() as usize;
            self.buffer.consume(len_byte_count);
        }

        // Read the packet bytes
        let mut packet_buf: Vec<u8> = vec![0; packet_len as usize];
        self.buffer.read_exact(&mut packet_buf).await?;
        let mut packet_cursor = Cursor::new(packet_buf);

        // Check if the packet is compressed
        if self.is_compressed() && 0 != u32::var_decode(&mut packet_cursor)? {
            // Decompress the packet
            let mut decompressor = ZlibDecoder::new(packet_cursor.remaining_slice());
            let mut decompressed = Vec::new();
            decompressor.read_to_end(&mut decompressed).await?;

            // Read the packet from the decompressed data
            let decompressed_len = decompressed.len();
            let mut cursor = Cursor::new(decompressed);

            let packet = <S as State<V>>::Clientbound::decode(&mut cursor);

            #[cfg(feature = "debug")]
            Self::trace_packet(&packet, &cursor);

            // Check if the packet is a bundle and read the bundled packets
            Self::read_bundle(packet_len, &mut cursor, &mut self.packet_buffer);

            // Return the packet
            Ok(packet?)
        } else {
            // Read the packet
            let packet = <S as State<V>>::Clientbound::decode(&mut packet_cursor);

            #[cfg(feature = "debug")]
            Self::trace_packet(&packet, &packet_cursor);

            // Check if the packet is a bundle and read the bundled packets
            Self::read_bundle(packet_len, &mut packet_cursor, &mut self.packet_buffer);

            // Return the packet
            Ok(packet?)
        }
    }

    /// Extracts packets from a bundle and adds them to the packet buffer.
    fn read_bundle(
        packet_len: u32,
        packet_cursor: &mut Cursor<Vec<u8>>,
        packet_buffer: &mut VecDeque<<S as State<V>>::Clientbound>,
    ) {
        while packet_len > u32::try_from(packet_cursor.position()).expect("Bundle packet too long")
        {
            let packet = <S as State<V>>::Clientbound::decode(packet_cursor);

            #[cfg(feature = "debug")]
            Self::trace_packet(&packet, packet_cursor);

            match packet {
                Ok(packet) => packet_buffer.push_back(packet),
                Err(err) => {
                    #[cfg(feature = "debug")]
                    if let DecodeError::Io(err) = err {
                        if !matches!(err.kind(), std::io::ErrorKind::UnexpectedEof) {
                            log::error!("Error reading bundled packet: {err:?}");
                        }
                    } else {
                        log::error!("Error reading bundled packet: {err:?}");
                    }
                    return;
                }
            }
        }

        #[cfg(feature = "debug")]
        log::trace!("Packet buffer len: {}", packet_buffer.len());
    }

    /// Logs the packet if the debug feature is enabled.
    #[cfg(feature = "debug")]
    fn trace_packet(
        packet: &Result<<S as State<V>>::Clientbound, DecodeError>,
        cursor: &Cursor<Vec<u8>>,
    ) {
        let mut string = format!("{:?}", packet);
        if string.len() > 100 {
            string.truncate(97);
            string.push_str("...");
        }

        if packet.is_ok() {
            log::trace!("Read packet: {}", string);
        } else {
            log::error!("Read packet: {}", string);
            log::trace!("Read buffer: {:?}", cursor.get_ref());
        }
    }

    /// Returns true if the connection is compressed
    pub fn is_compressed(&self) -> bool {
        if let Some(val) = self.compression {
            val >= 0
        } else {
            false
        }
    }

    /// Returns the peer address of the connection.
    pub fn peer_addr(&self) -> Option<SocketAddr> { self.stream.peer_addr().ok() }

    /// Converts this connection into a connection with a different state.
    pub fn into<S2>(self) -> Connection<V, S2>
    where
        S2: State<V>,
    {
        Connection {
            _version: PhantomData,
            _state: PhantomData,
            hostname: self.hostname,
            port: self.port,
            compression: self.compression,
            packet_buffer: VecDeque::new(),
            buffer: self.buffer,
            stream: self.stream,
        }
    }
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Error encoding packet: {0}")]
    Encode(#[from] EncodeError),
    #[error("Error decoding packet: {0}")]
    Decode(#[from] DecodeError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParsePort(#[from] std::num::ParseIntError),
    #[error("No ip address found")]
    NoAddressFound,
    #[error("Unexpected packet")]
    UnexpectedPacket,
    #[error("Connection Closed")]
    Closed,
    #[error("Disconnected: {0:?}")]
    Disconnected(FormattedText),
}

impl<V: Version, S: State<V>> TryFrom<std::net::TcpStream> for Connection<V, S> {
    type Error = std::io::Error;

    fn try_from(value: std::net::TcpStream) -> Result<Self, Self::Error> {
        TcpStream::try_from(value)?.try_into()
    }
}

impl<V: Version, S: State<V>> TryFrom<TcpStream> for Connection<V, S> {
    type Error = std::io::Error;

    fn try_from(stream: TcpStream) -> Result<Self, Self::Error> {
        stream.set_nodelay(true)?;
        Ok(Connection {
            _version: PhantomData,
            _state: PhantomData,
            hostname: stream.peer_addr()?.ip().to_string().into(),
            port: stream.peer_addr()?.port(),
            compression: None,
            packet_buffer: VecDeque::new(),
            buffer: BufReader::with_capacity(Self::BUFFER_SIZE, stream.clone()),
            stream,
        })
    }
}

impl<V: Version, S: State<V>> From<Connection<V, S>> for TcpStream {
    fn from(conn: Connection<V, S>) -> Self { conn.stream }
}
