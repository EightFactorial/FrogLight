//! TODO

use core::error::Error;
use std::io::Cursor;

use async_net::TcpStream;
use async_trait::async_trait;
use froglight_common::version::Version;
use froglight_io::version::{FrogReadVersion, FrogWriteVersion};
use futures_lite::{AsyncBufReadExt, io::BufReader};

use crate::connection::{RawConnection, raw::RawPacketVersion};

/// A marker struct for [`RawPacketVersion`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IoPacket;

impl<V: Version, T: FrogReadVersion<V> + FrogWriteVersion<V> + Send + Sync + 'static>
    RawPacketVersion<V, IoPacket> for T
{
    async fn read_packet<'data>(bytes: &'data [u8]) -> Result<(Self, &'data [u8]), Box<dyn Error>> {
        let mut cursor = Cursor::new(bytes);
        let packet = <T as FrogReadVersion<V>>::frog_read(&mut cursor)?;
        Ok((packet, &bytes[cursor.position() as usize..]))
    }

    async fn write_packet(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::new();
        <T as FrogWriteVersion<V>>::frog_write(&self, &mut buffer)?;
        Ok(buffer)
    }
}

// -------------------------------------------------------------------------------------------------

/// The default [`RawConnection`] implementation.
#[derive(Debug)]
pub struct IoTransport {
    stream: BufReader<TcpStream>,
    compression: Option<i32>,
}

#[async_trait]
impl RawConnection for IoTransport {
    /// Set the compression threshold for the connection.
    #[inline]
    async fn set_compression(&mut self, threshold: Option<i32>) { self.compression = threshold }

    /// Get the compression threshold for the connection.
    #[inline]
    async fn get_compression(&self) -> Option<i32> { self.compression }

    /// Read data from the connection.
    async fn read_raw<'data>(&'data mut self) -> Result<&'data [u8], Box<dyn Error>> { todo!() }

    /// Consume data from the connection.
    #[inline]
    async fn consume_raw(&mut self, count: usize) { self.stream.consume(count); }

    /// Write data into the connection.
    async fn write_raw(&mut self, _buff: &[u8]) -> Result<(), Box<dyn Error>> { todo!() }
}
