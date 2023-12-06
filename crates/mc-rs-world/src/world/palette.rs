use std::io::Cursor;

use mc_rs_protocol::buffer::VarDecode;

use super::{container::traits::ContainerType, tasks::ChunkDecodeError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Palette {
    Single(u32),
    Vector(Vec<u32>),
    Global,
}

impl Default for Palette {
    fn default() -> Self { Self::Single(0u32) }
}

impl Palette {
    pub(super) async fn decode_palette<T: ContainerType>(
        bits: &u8,
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Self, ChunkDecodeError> {
        match T::palette_type(bits) {
            Palette::Single(_) => Ok(Palette::Single(
                u32::var_decode(cursor).map_err(|_| ChunkDecodeError::InvalidPalette)?,
            )),
            Palette::Vector(_) => Ok(Palette::Vector(
                Vec::<u32>::var_decode(cursor).map_err(|_| ChunkDecodeError::InvalidPalette)?,
            )),
            Palette::Global => Ok(Palette::Global),
        }
    }
}
