use std::io::Cursor;

use mc_rs_protocol::buffer::VarDecode;

use super::{container::traits::ContainerType, tasks::ChunkDecodeError};

/// A [`Palette`] is used to store the kinds of blocks in a [`Container`].
///
/// For [`Single`](Palette::Single), the [`Container`](super::container::Container) contains no
/// data, all positions in the [`Container`](super::container::Container) are the same as the
/// [`Palette`] value.
///
/// For [`Vector`](Palette::Vector), the [`Container`](super::container::Container) contains indices
/// into the [`Palette`] for each block.
///
/// For [`Global`](Palette::Global), the [`Container`](super::container::Container) contains the
/// global index for each block itself.
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
        bits: &usize,
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

    /// Get the data at the given index in the [`Palette`].
    ///
    /// Returns `None` if the [`Palette`] is [`Palette::Global`].
    pub fn get(&self, index: usize) -> Option<&u32> {
        match self {
            Palette::Single(val) => Some(val),
            Palette::Vector(vec) => vec.get(index),
            Palette::Global => None,
        }
    }
}
