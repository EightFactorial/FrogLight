#![allow(unused_variables)]
#![allow(dead_code)]

use std::{collections::VecDeque, io::Cursor, marker::PhantomData};

use async_compression::futures::{bufread::ZlibDecoder, write::ZlibEncoder};
use async_net::{AsyncToSocketAddrs, SocketAddr, TcpStream};
use futures_lite::{io::BufReader, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use thiserror::Error;

use crate::{
    buffer::{Decode, DecodeError, Encode, EncodeError, VarDecode, VarEncode},
    versions::state::Handshake,
    State, Version,
};

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Connection<V: Version, S: State<V>> {
    _version: PhantomData<V>,
    _state: PhantomData<S>,
    pub hostname: String,
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
    pub async fn new(version: V, address: impl Into<String>) -> Result<Self, ConnectionError> {
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
    /// Create a new connection from a hostname and port.
    pub async fn new_from(hostname: &str, port: u16) -> Result<Self, ConnectionError> {
        let stream = TcpStream::connect((hostname, port)).await?;

        Ok(Self {
            _version: PhantomData,
            _state: PhantomData,
            hostname: hostname.to_owned(),
            port,
            compression: None,
            packet_buffer: VecDeque::new(),
            buffer: BufReader::new(stream.clone()),
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
        let mut buf = Vec::new();
        packet.into().encode(&mut buf)?;

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
            return Ok(packet);
        }

        // Read the packet from the stream
        let mut buffer = self.buffer.fill_buf().await?;

        // TODO: Decryption

        // Read the length of the packet
        let len = u32::var_decode(&mut buffer)? as usize;

        // Take the packet bytes
        let mut buf: Vec<u8> = Vec::with_capacity(len);
        buf.extend_from_slice(&buffer[..len]);
        let mut cursor = Cursor::new(buf);

        // Decompress the packet if needed
        if self.is_compressed() && 0 == u32::var_decode(&mut cursor)? {
            // Decompress the packet
            let mut decompressor = ZlibDecoder::new(cursor.remaining_slice());
            let mut decompressed = Vec::new();
            decompressor.read_to_end(&mut decompressed).await?;

            // Read the packet from the decompressed data
            let decompressed_len = decompressed.len();
            let mut cursor = Cursor::new(decompressed);
            let packet = <S as State<V>>::Clientbound::decode(&mut cursor);

            // Try to read more packets if the length doesn't match
            while decompressed_len > cursor.position() as usize {
                match <S as State<V>>::Clientbound::decode(&mut cursor) {
                    Ok(packet) => self.packet_buffer.push_back(packet),
                    Err(err) => {
                        return Err(err.into());
                    }
                }
            }

            // Consume the length from the buffer
            self.buffer.consume(len);

            // Return the packet
            Ok(packet?)
        } else {
            // Read the packet
            let packet = <S as State<V>>::Clientbound::decode(&mut cursor);

            // Try to read more packets if the length doesn't match
            while len > cursor.position() as usize {
                match <S as State<V>>::Clientbound::decode(&mut cursor) {
                    Ok(packet) => self.packet_buffer.push_back(packet),
                    Err(err) => {
                        return Err(err.into());
                    }
                }
            }

            // Consume the length from the buffer
            self.buffer.consume(cursor.position() as usize);

            // Return the packet
            Ok(packet?)
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
    #[error(transparent)]
    Encode(#[from] EncodeError),
    #[error(transparent)]
    Decode(#[from] DecodeError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParsePort(#[from] std::num::ParseIntError),
    #[error("No ip address found")]
    NoAddressFound,
    #[error("Unexpected packet")]
    UnexpectedPacket,
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
            hostname: stream.peer_addr()?.ip().to_string(),
            port: stream.peer_addr()?.port(),
            compression: None,
            packet_buffer: VecDeque::new(),
            buffer: BufReader::new(stream.clone()),
            stream,
        })
    }
}

impl<V: Version, S: State<V>> From<Connection<V, S>> for TcpStream {
    fn from(conn: Connection<V, S>) -> Self { conn.stream }
}
