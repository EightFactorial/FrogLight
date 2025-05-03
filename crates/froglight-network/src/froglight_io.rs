//! TODO

use core::net::SocketAddr;
use std::io::Cursor;

use async_net::TcpStream;
use async_trait::async_trait;
use froglight_common::version::Version;
use froglight_io::{
    prelude::{FrogVarRead, FrogVarWrite},
    version::{FrogReadVersion, FrogWriteVersion},
};
use froglight_packet::state::{Handshake, ValidState};
use futures_lite::{AsyncReadExt, AsyncWriteExt};

use crate::{
    connection::{RawConnection, raw::RawPacketVersion, state::ConnectionError},
    prelude::ClientConnection,
};

impl<V: ValidState<Handshake>> ClientConnection<V, Handshake> {
    /// Connect to a server at the given address,
    /// resolved using the provided
    /// [`FroglightResolver`](crate::prelude::FroglightResolver).
    #[cfg(feature = "resolver")]
    pub async fn connect(
        addr: &str,
        resolver: &crate::prelude::FroglightResolver,
    ) -> Result<Self, std::io::Error> {
        IoTransport::<TcpStream>::connect(addr, resolver).await.map(Self::from_raw)
    }

    /// Connect to a server at the given address,
    /// resolved using the default system resolver.
    pub async fn connect_system(addr: &str) -> Result<Self, std::io::Error> {
        IoTransport::<TcpStream>::connect_system(addr).await.map(Self::from_raw)
    }

    /// Connect to a server at the given [`SocketAddr`].
    pub async fn connect_to(socket: SocketAddr) -> Result<Self, std::io::Error> {
        IoTransport::<TcpStream>::connect_to(socket).await.map(Self::from_raw)
    }
}

// -------------------------------------------------------------------------------------------------

/// A marker struct for [`RawPacketVersion`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IoPacket;

impl<V: Version, T: FrogReadVersion<V> + FrogWriteVersion<V> + Send + Sync + 'static>
    RawPacketVersion<V, IoPacket> for T
{
    async fn read_packet<'a, C: RawConnection + ?Sized>(
        conn: &'a mut C,
        buf: &'a mut Vec<u8>,
    ) -> Result<Self, ConnectionError> {
        buf.clear();

        conn.read_packet(buf).await?;
        let mut cursor = Cursor::new(buf.as_mut_slice());
        let result = T::frog_read(&mut cursor);
        result.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))
    }

    async fn write_packet<'a, C: RawConnection + ?Sized>(
        &'a self,
        conn: &'a mut C,
        buf: &'a mut Vec<u8>,
    ) -> Result<(), ConnectionError> {
        buf.clear();

        let result = self.frog_write(buf);
        result.map_err(|err| ConnectionError::WriteRawPacket(Box::new(err)))?;
        conn.write_packet(buf).await
    }
}

// -------------------------------------------------------------------------------------------------

/// The default [`RawConnection`] implementation.
pub struct IoTransport<S> {
    stream: S,
    peer: SocketAddr,
    compression: Option<i32>,
    _compress_buf: Vec<u8>,
}

impl<S> IoTransport<S> {
    /// Creates a new [`IoTransport`] instance.
    #[must_use]
    pub const fn wrap(stream: S, peer: SocketAddr) -> Self {
        Self { stream, peer, compression: None, _compress_buf: Vec::new() }
    }
}

#[async_trait]
impl<S: AsyncPeekExt + AsyncWriteExt + Send + Sync + Unpin + 'static> RawConnection
    for IoTransport<S>
{
    #[inline]
    async fn peer_addr(&self) -> Result<SocketAddr, ConnectionError> { Ok(self.peer) }

    #[inline]
    async fn set_compression(&mut self, threshold: Option<i32>) { self.compression = threshold }

    #[inline]
    async fn get_compression(&self) -> Option<i32> { self.compression }

    // TODO: Decryption + Decompression
    async fn read_packet(&mut self, buf: &mut Vec<u8>) -> Result<(), ConnectionError> {
        // Get the size of the packet and the size of the packet size
        let mut len_buf = Cursor::new([0u8; 5]);
        self.peek_raw(len_buf.get_mut().as_mut_slice()).await?;
        let len = <u32 as FrogVarRead>::frog_var_read(&mut len_buf);
        let len = len.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))? as usize;
        let len_size = len_buf.position() as usize;

        // Make sure the buffer can hold the packet, then read it
        buf.resize(buf.len().max(len + len_size), 0u8);
        let pbuf = &mut buf[..len + len_size];
        self.read_raw(pbuf).await?;

        // Create a cursor and skip the packet size
        let mut cursor = Cursor::new(pbuf);
        cursor.set_position(len_size as u64);

        if self.compression.is_some_and(|c| c >= 0) {
            let size = <u32 as FrogVarRead>::frog_var_read(&mut cursor);
            let size = size.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))? as usize;

            if size != 0 {
                todo!("Packet Decompression")
            }
        }

        // Move the packet to the beginning of the buffer and set the size.
        let position = cursor.position() as usize;
        buf.copy_within(position.., 0);
        buf.truncate(buf.len() - position);

        Ok(())
    }

    // TODO: Compression + Encryption
    async fn write_packet(&mut self, buf: &[u8]) -> Result<(), ConnectionError> {
        // Get the packet length and prefix length
        let packet_len = buf.len();
        let prefix_len = packet_len + packet_len.frog_var_len();

        if self.compression.is_some_and(|c| c <= prefix_len.try_into().unwrap_or_default()) {
            todo!("Packet Compression")
        } else {
            let mut prefix_buf = Vec::<u8>::with_capacity(8);

            if self.compression.is_some() {
                // Write the packet length and a `0` byte
                let result = FrogVarWrite::frog_var_write(&(packet_len + 1), &mut prefix_buf);
                result.map_err(|err| ConnectionError::WriteRawPacket(Box::new(err)))?;
                prefix_buf.push(0);
            } else {
                // Write the packet length
                let result = FrogVarWrite::frog_var_write(&packet_len, &mut prefix_buf);
                result.map_err(|err| ConnectionError::WriteRawPacket(Box::new(err)))?;
            }

            self.write_raw(&prefix_buf).await?;
            self.write_raw(buf).await
        }
    }

    async fn read_raw(&mut self, buf: &mut [u8]) -> Result<(), ConnectionError> {
        let result = self.stream.read_exact(buf).await;
        result.map_err(|err| ConnectionError::ReadRawConnection(Box::new(err)))
    }

    async fn peek_raw(&mut self, buf: &mut [u8]) -> Result<usize, ConnectionError> {
        let result = self.stream.peek(buf).await;
        result.map_err(|err| ConnectionError::ReadRawConnection(Box::new(err)))
    }

    async fn write_raw(&mut self, buf: &[u8]) -> Result<(), ConnectionError> {
        let result = self.stream.write_all(buf).await;
        result.map_err(|err| ConnectionError::WriteRawConnection(Box::new(err)))
    }
}

// -------------------------------------------------------------------------------------------------

/// Extension trait for [`AsyncReadExt`]
pub trait AsyncPeekExt: AsyncReadExt {
    /// Peek at the next bytes in the stream without consuming them.
    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = std::io::Result<usize>> + Send;
}

impl AsyncPeekExt for TcpStream {
    #[inline]
    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = std::io::Result<usize>> + Send {
        self.peek(buf)
    }
}

impl IoTransport<TcpStream> {
    /// Create an [`IoTransport`] from an address resolved
    /// using the provided
    /// [`FroglightResolver`](crate::prelude::FroglightResolver).
    #[inline]
    #[cfg(feature = "resolver")]
    pub async fn connect(
        addr: &str,
        resolver: &crate::prelude::FroglightResolver,
    ) -> Result<Self, std::io::Error> {
        Self::connect_to(resolver.lookup_minecraft(addr).await?).await
    }

    /// Create an [`IoTransport`] from an address resolved
    /// using the system's default resolver.
    pub async fn connect_system(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr).await?;
        let peer = stream.peer_addr()?;
        stream.set_nodelay(true)?;
        Ok(Self::wrap(stream, peer))
    }

    /// Create an [`IoTransport`] from a [`SocketAddr`].
    pub async fn connect_to(socket: SocketAddr) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(socket).await?;
        let peer = stream.peer_addr()?;
        stream.set_nodelay(true)?;
        Ok(Self::wrap(stream, peer))
    }
}
