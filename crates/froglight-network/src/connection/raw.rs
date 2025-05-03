//! [`RawConnection`], [`RawPacket`], and [`RawPacketVersion`]

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};

use async_trait::async_trait;
use froglight_common::version::Version;

use super::state::ConnectionError;

/// A dyn-compatible connection, either to a server or client.
#[async_trait]
pub trait RawConnection: Send + 'static {
    /// Set the compression threshold for the connection.
    async fn set_compression(&mut self, threshold: Option<i32>);
    /// Get the compression threshold for the connection.
    async fn get_compression(&self) -> Option<i32>;

    /// Read a packet from the connection, returning the number of bytes read.
    ///
    /// This should do all processing and return the final packet data.
    async fn read_packet(&mut self, buf: &mut Vec<u8>) -> Result<(), ConnectionError>;
    /// Write a packet to the connection, returning the number of bytes written.
    ///
    /// This should do all processing and write the final packet data.
    async fn write_packet(&mut self, buf: &[u8]) -> Result<(), ConnectionError>;

    /// Read data from the connection, filling the provided buffer.
    ///
    /// This should not perform any processing and return the raw data.
    async fn read_raw(&mut self, buf: &mut [u8]) -> Result<(), ConnectionError>;
    /// Peek data from the connection, returning the number of bytes read.
    ///
    /// This should not perform any processing and return the raw data.
    async fn peek_raw(&mut self, buf: &mut [u8]) -> Result<usize, ConnectionError>;

    /// Write data to the connection.
    ///
    /// This should not perform any processing and write the raw data.
    async fn write_raw(&mut self, buf: &[u8]) -> Result<(), ConnectionError>;
}

// -------------------------------------------------------------------------------------------------

/// A packet that can be read from and written to bytes.
pub trait RawPacket: Sized + Send + 'static {
    /// Read a packet from the given bytes.
    fn read_packet<'a, C: RawConnection + ?Sized>(
        conn: &'a mut C,
        buf: &'a Vec<u8>,
    ) -> impl Future<Output = Result<Self, ConnectionError>> + Send + 'a;
    /// Write the packet into the given buffer.
    fn write_packet<'a, C: RawConnection + ?Sized>(
        &'a self,
        conn: &'a mut C,
        buf: &'a Vec<u8>,
    ) -> impl Future<Output = Result<(), ConnectionError>> + Send + 'a;
}

/// A packet that can be read from and written to bytes.
pub trait RawPacketVersion<V: Version, M: 'static>: Sized + Send + 'static {
    /// Read a packet from the given bytes.
    fn read_packet<'a, C: RawConnection + ?Sized>(
        conn: &'a mut C,
        buf: &'a mut Vec<u8>,
    ) -> impl Future<Output = Result<Self, ConnectionError>> + Send + 'a;
    /// Write the packet into the given buffer.
    fn write_packet<'a, C: RawConnection + ?Sized>(
        &'a self,
        conn: &'a mut C,
        buf: &'a mut Vec<u8>,
    ) -> impl Future<Output = Result<(), ConnectionError>> + Send + 'a;
}

impl<V: Version, T: RawPacket> RawPacketVersion<V, ()> for T {
    /// Read a packet from the given bytes.
    #[inline]
    fn read_packet<'a, C: RawConnection + ?Sized>(
        conn: &'a mut C,
        buf: &'a mut Vec<u8>,
    ) -> impl Future<Output = Result<Self, ConnectionError>> + Send + 'a {
        <Self as RawPacket>::read_packet(conn, buf)
    }

    /// Write the packet into the given buffer.
    #[inline]
    fn write_packet<'a, C: RawConnection + ?Sized>(
        &'a self,
        conn: &'a mut C,
        buf: &'a mut Vec<u8>,
    ) -> impl Future<Output = Result<(), ConnectionError>> + Send + 'a {
        <Self as RawPacket>::write_packet(self, conn, buf)
    }
}
