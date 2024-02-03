use bevy::reflect::Reflect;
use froglight_protocol::io::FrogRead;

use crate::world::chunk::ChunkDecodeError;

/// Storage for the kinds of blocks in a [`Container`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum Palette {
    /// A single block ID.
    Single(u32),
    /// A vector of block IDs.
    Vector(Vec<u32>),
    /// A global palette.
    Global,
}

impl Default for Palette {
    fn default() -> Self { Self::Single(0u32) }
}

impl Palette {
    /// Decodes a [`Palette`] from a buffer.
    pub(crate) fn decode(
        &self,
        buf: &mut std::io::Cursor<&[u8]>,
    ) -> Result<Self, ChunkDecodeError> {
        match self {
            Palette::Single(_) => Ok(Palette::Single(u32::fg_read(buf)?)),
            Palette::Vector(_) => Ok(Palette::Vector(Vec::<u32>::fg_read(buf)?)),
            Palette::Global => Ok(Palette::Global),
        }
    }
}
