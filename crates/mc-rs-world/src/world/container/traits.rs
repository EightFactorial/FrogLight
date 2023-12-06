use std::io::Cursor;

use futures_lite::Future;
use mc_rs_protocol::buffer::Decode;

use crate::world::{palette::Palette, tasks::ChunkDecodeError};

use super::Container;

pub trait ContainerType: Sized + Default {
    fn decode_container(
        cursor: &mut Cursor<&[u8]>,
    ) -> impl Future<Output = Result<Container<Self>, ChunkDecodeError>> {
        async {
            let bits = u8::decode(cursor).map_err(|_| ChunkDecodeError::InvalidContainer)?;

            Ok(Container::<Self> {
                palette: Palette::decode_palette::<Self>(&bits, cursor).await?,
                data: Vec::<u64>::decode(cursor).map_err(|_| ChunkDecodeError::InvalidContainer)?,
                bits,
                ..Container::<Self>::default()
            })
        }
    }

    fn palette_type(bits: &u8) -> Palette;
}
