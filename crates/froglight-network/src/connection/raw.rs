//! [`RawConnection`], [`RawPacket`], and [`RawPacketVersion`]

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};
use core::error::Error;

use async_trait::async_trait;
use froglight_common::version::Version;

/// A dyn-compatible connection, either to a server or client.
#[async_trait]
pub trait RawConnection: Send + 'static {
    /// Set the compression threshold for the connection.
    async fn set_compression(&mut self, threshold: Option<i32>);
    /// Get the compression threshold for the connection.
    async fn get_compression(&self) -> Option<i32>;

    /// Read data from the connection.
    async fn read_raw<'data>(&'data mut self) -> Result<&'data [u8], Box<dyn Error>>;
    /// Consume data from the connection.
    async fn consume_raw(&mut self, count: usize);

    /// Write data into the connection.
    async fn write_raw(&mut self, buff: &[u8]) -> Result<(), Box<dyn Error>>;
}

// -------------------------------------------------------------------------------------------------

/// A packet that can be read from and written to bytes.
pub trait RawPacket: Sized + Send + 'static {
    /// Read a packet from the given bytes.
    fn read_packet<'data>(
        bytes: &'data [u8],
    ) -> impl Future<Output = Result<(Self, &'data [u8]), Box<dyn Error>>> + Send + 'data;
    /// Write the packet into the given buffer.
    fn write_packet(&self) -> impl Future<Output = Result<Vec<u8>, Box<dyn Error>>> + Send;
}

/// A packet that can be read from and written to bytes.
pub trait RawPacketVersion<V: Version, M: 'static>: Sized + Send + 'static {
    /// Read a packet from the given bytes.
    fn read_packet<'data>(
        bytes: &'data [u8],
    ) -> impl Future<Output = Result<(Self, &'data [u8]), Box<dyn Error>>> + Send + 'data;
    /// Write the packet into the given buffer.
    fn write_packet(&self) -> impl Future<Output = Result<Vec<u8>, Box<dyn Error>>> + Send;
}

impl<V: Version, T: RawPacket> RawPacketVersion<V, ()> for T {
    #[inline]
    fn read_packet<'data>(
        bytes: &'data [u8],
    ) -> impl Future<Output = Result<(Self, &'data [u8]), Box<dyn Error>>> + Send + 'data {
        Self::read_packet(bytes)
    }

    #[inline]
    fn write_packet(&self) -> impl Future<Output = Result<Vec<u8>, Box<dyn Error>>> + Send {
        self.write_packet()
    }
}
