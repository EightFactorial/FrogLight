use std::io::Cursor;

use froglight_common::version::Version;
use froglight_io::version::{FrogReadVersion, FrogWriteVersion};

use crate::connection::{RawConnection, raw::RawPacketVersion, state::ConnectionError};

/// An adapter between [`RawPacketVersion`]
/// and [`FrogReadVersion`] / [`FrogWriteVersion`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IoAdapter;

impl<V: Version, T: FrogReadVersion<V> + FrogWriteVersion<V> + Send + Sync + 'static>
    RawPacketVersion<V, IoAdapter> for T
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
