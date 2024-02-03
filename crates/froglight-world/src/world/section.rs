//! A [`Section`] is a 16x16x16 section of a [`Chunk`](super::Chunk).

use std::io::Cursor;

use bevy::reflect::Reflect;
use froglight_protocol::io::FrogRead;

use super::{chunk::ChunkDecodeError, BiomeContainer, BlockContainer, Chunk, ChunkDataContainer};

/// A [`Section`] is a 16x16x16 section of a [`Chunk`](super::Chunk).
///
/// A [`Section`] contains a counter for the number of non-air blocks in the
/// section, and [`Containers`](super::container::Container) for blocks and
/// biomes.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct Section {
    /// The number of non-air blocks in the section.
    pub block_count: u16,
    /// The block data stored in the section.
    pub blocks: ChunkDataContainer<BlockContainer>,
    /// The biome data stored in the section.
    pub biomes: ChunkDataContainer<BiomeContainer>,
}

impl Section {
    /// The width of a [`Section`].
    pub const WIDTH: usize = Chunk::WIDTH;
    /// The depth of a [`Section`].
    pub const DEPTH: usize = Chunk::DEPTH;
    /// The height of a [`Section`].
    pub const HEIGHT: usize = 16;

    /// The total volume of a [`Section`].
    pub const VOLUME: usize = Self::WIDTH * Self::DEPTH * Self::HEIGHT;

    /// Decodes a [`Section`] from a buffer.
    pub(crate) fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, ChunkDecodeError> {
        Ok(Self {
            block_count: u16::fg_read(buf)?,
            blocks: ChunkDataContainer::decode(buf)?,
            biomes: ChunkDataContainer::decode(buf)?,
        })
    }
}
