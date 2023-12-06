use std::io::Cursor;

use mc_rs_protocol::buffer::Decode;

use crate::world::{palette::Palette, tasks::ChunkDecodeError};

use super::{traits::ContainerType, Container};

/// A container for biome data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BiomeContainer;

impl ContainerType for BiomeContainer {
    async fn decode_container(
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<super::Container<Self>, ChunkDecodeError> {
        let bits = u8::decode(cursor).map_err(|_| ChunkDecodeError::InvalidContainer)?;

        Ok(Container::<Self> {
            palette: Palette::decode_palette::<Self>(&bits, cursor).await?,
            bits,
            ..Default::default()
        })
    }

    fn palette_type(bits: &u8) -> Palette {
        match bits {
            0 => Palette::Single(0u32),
            1..=3 => Palette::Vector(Vec::<u32>::new()),
            _ => Palette::Global,
        }
    }
}
