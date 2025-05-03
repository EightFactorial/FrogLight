//! TODO

use std::io::Cursor;

use async_net::TcpStream;
use async_trait::async_trait;
use froglight_common::version::Version;
use froglight_io::version::{FrogReadVersion, FrogWriteVersion};
use futures_lite::{AsyncReadExt, AsyncWriteExt};

use crate::connection::{RawConnection, raw::RawPacketVersion, state::ConnectionError};

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
        if let Err(err) = conn.read_packet(buf).await {
            buf.clear();
            return Err(ConnectionError::ReadRawConnection(Box::new(err)));
        }

        let mut cursor = Cursor::new(buf.as_mut_slice());
        let result = T::frog_read(&mut cursor);
        buf.clear();
        result.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))
    }

    async fn write_packet<'a, C: RawConnection + ?Sized>(
        &'a self,
        conn: &'a mut C,
        buf: &'a mut Vec<u8>,
    ) -> Result<(), ConnectionError> {
        if let Err(err) = T::frog_write(self, buf) {
            buf.clear();
            return Err(ConnectionError::WriteRawPacket(Box::new(err)));
        }

        let result = conn.write_packet(buf).await;
        buf.clear();
        result.map(|_| ())
    }
}

// -------------------------------------------------------------------------------------------------

/// The default [`RawConnection`] implementation.
pub struct IoTransport {
    stream: TcpStream,
    compression: Option<i32>,
}

#[async_trait]
impl RawConnection for IoTransport {
    #[inline]
    async fn set_compression(&mut self, threshold: Option<i32>) { self.compression = threshold }

    #[inline]
    async fn get_compression(&self) -> Option<i32> { self.compression }

    async fn read_packet(&mut self, _buf: &mut Vec<u8>) -> Result<(), ConnectionError> {
        // TODO: Decryption + Decompression
        todo!()
    }

    async fn write_packet(&mut self, _buf: &[u8]) -> Result<usize, ConnectionError> {
        // TODO: Compression + Encryption
        todo!()
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
