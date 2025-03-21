use std::io::Cursor;

use froglight_common::version::Version;
use froglight_io::{
    prelude::*,
    version::{FrogReadVersion, FrogWriteVersion},
};
use futures_lite::{AsyncReadExt, AsyncWriteExt};

use super::RawConnection;

macro_rules! fn_with_generics {
    // TODO: Decryption
    (@read $name:ident, $($tmpl:tt)*) => {
        pub(super) async fn $name $($tmpl)* (
            raw: &mut RawConnection,
        ) -> Result<T, ReadError> {
            let mut len_buf = [0u8; 5];
            raw.stream.peek(&mut len_buf).await?;
            let mut len_cursor = Cursor::new(len_buf.as_slice());

            #[expect(clippy::cast_possible_truncation)]
            let mut packet_buf: Vec<u8> =
                Vec::with_capacity(usize::frog_var_read(&mut len_cursor)? + len_cursor.position() as usize);
            raw.stream.read_exact(packet_buf.as_mut_slice()).await?;

            #[expect(clippy::cast_possible_truncation)]
            let mut packet_cursor = Cursor::new(&packet_buf[len_cursor.position() as usize..]);
            if raw.compression().is_some_and(|c| c >= 0) && 0 != u32::frog_var_read(&mut packet_cursor)? {
                unimplemented!("Packet Decompression")
            } else {
                T::frog_read(&mut packet_cursor)
            }
        }
    };
    // TODO: Decryption
    (@write $name:ident, $($tmpl:tt)*) => {
        pub(super) async fn $name $($tmpl)* (
            raw: &mut RawConnection,
            val: &T,
        ) -> Result<(), WriteError> {
            let packet_len = val.frog_len();
            let prefixed_len = packet_len + packet_len.frog_var_len();

            let compression = raw.compression();
            if compression.is_some_and(|c| i32::try_from(prefixed_len).unwrap_or_default() >= c) {
                unimplemented!("Packet Compression");
            } else {
                let mut buf = Vec::with_capacity(prefixed_len);

                if compression.is_some() {
                    (packet_len + 1).frog_var_write(&mut buf)?;
                    FrogWrite::frog_write(&0u8, &mut buf)?;
                } else {
                    packet_len.frog_var_write(&mut buf)?;
                }

                val.frog_write(&mut buf)?;
                raw.stream.write_all(&buf).await.map_err(WriteError::Io)
            }
        }

    };
}

fn_with_generics!(@read read_type, <T: FrogRead + Send + Sync>);
fn_with_generics!(@read read_type_version, <T: FrogReadVersion<V> + Send + Sync, V: Version>);

fn_with_generics!(@write write_type, <T: FrogWrite + Send + Sync>);
fn_with_generics!(@write write_type_version, <T: FrogWriteVersion<V> + Send + Sync, V: Version>);
