#![allow(unused_variables)]
#![allow(dead_code)]

use std::{io::Cursor, marker::PhantomData};

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
    pub compression: Option<i32>,
    buffer: BufReader<TcpStream>,
    stream: TcpStream,
}

impl<V: Version> Connection<V, Handshake>
where
    Handshake: State<V>,
{
    /// Create a new connection from an address.
    pub async fn new(version: V, mut address: &str) -> Result<Self, ConnectionError> {
        if address.starts_with("http://") {
            address = &address[7..];
        } else if address.starts_with("https://") {
            address = &address[8..];
        } else if address.starts_with("tcp://") {
            address = &address[6..];
        }

        if let Some(colon) = address.find(':') {
            let (address, port) = address.split_at(colon);
            let port: u16 = port[1..].parse()?;

            Self::new_from((address, port)).await
        } else {
            Self::new_from((address, 25565)).await
        }
    }
}

impl<V: Version, S: State<V>> Connection<V, S> {
    /// Create a new connection from an address.
    pub async fn new_from(address: impl AsyncToSocketAddrs) -> Result<Self, ConnectionError> {
        let mut addresses = address.to_socket_addrs().await?;
        let address = addresses.next().ok_or(ConnectionError::NoAddressFound)?;
        Self::from_sock(address).await
    }

    /// Create a new connection from a socket address.
    pub async fn from_sock(address: SocketAddr) -> Result<Self, ConnectionError> {
        let stream = TcpStream::connect(address).await?;
        Ok(stream.into())
    }

    /// Sends a packet to the server.
    pub async fn send_packet(
        &mut self,
        packet: impl Into<<S as State<V>>::Serverbound>,
    ) -> Result<(), EncodeError> {
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
    fn add_length(buf: &mut Vec<u8>) -> Result<(), EncodeError> {
        let mut len_buf = Vec::with_capacity(buf.len() + 2);
        buf.len().var_encode(&mut len_buf)?;
        len_buf.extend_from_slice(buf);
        *buf = len_buf;
        Ok(())
    }

    /// Receives a packet from the server.
    pub async fn receive_packet(&mut self) -> Result<<S as State<V>>::Clientbound, DecodeError> {
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
            let packet = <S as State<V>>::Clientbound::decode(&mut Cursor::new(decompressed))?;

            // Consume the length from the buffer
            self.buffer.consume(len);

            // Return the packet
            Ok(packet)
        } else {
            // Read the packet
            let packet = <S as State<V>>::Clientbound::decode(&mut cursor)?;

            // Consume the length from the buffer
            self.buffer.consume(cursor.position() as usize);

            // Return the packet
            Ok(packet)
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

    /// Converts this connection into a connection with a different state.
    pub fn into<S2>(self) -> Connection<V, S2>
    where
        S2: State<V>,
    {
        Connection {
            _version: PhantomData,
            _state: PhantomData,
            compression: self.compression,
            buffer: self.buffer,
            stream: self.stream,
        }
    }

    /// Converts another connection into this state.
    pub fn from<S2>(other: Connection<V, S2>) -> Self
    where
        S2: State<V>,
    {
        Connection {
            _version: PhantomData,
            _state: PhantomData,
            compression: other.compression,
            buffer: other.buffer,
            stream: other.stream,
        }
    }
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error parsing port: {0}")]
    ParsePort(#[from] std::num::ParseIntError),
    #[error("No ip address found")]
    NoAddressFound,
}

impl<V: Version, S: State<V>> TryFrom<std::net::TcpStream> for Connection<V, S> {
    type Error = std::io::Error;

    fn try_from(value: std::net::TcpStream) -> Result<Self, Self::Error> {
        Ok(TcpStream::try_from(value)?.into())
    }
}

impl<V: Version, S: State<V>> From<TcpStream> for Connection<V, S> {
    fn from(stream: TcpStream) -> Self {
        Connection {
            _version: PhantomData,
            _state: PhantomData,
            compression: None,
            buffer: BufReader::new(stream.clone()),
            stream,
        }
    }
}

impl<V: Version, S: State<V>> From<Connection<V, S>> for TcpStream {
    fn from(conn: Connection<V, S>) -> Self { conn.stream }
}
