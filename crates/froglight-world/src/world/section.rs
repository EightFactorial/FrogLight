//! A [`Section`] is a 16x16x16 section of a [`Chunk`](super::Chunk).

use std::io::Cursor;

use bevy_reflect::Reflect;
use froglight_core::data::SectionBlockPosition;
use froglight_protocol::io::FrogRead;

use super::{chunk::ChunkDecodeError, BiomeContainer, BlockContainer, Chunk, ChunkDataContainer};

/// A [`Section`] is a 16x16x16 section of a [`Chunk`](super::Chunk).
///
/// A [`Section`] contains a counter for the number of non-air blocks in the
/// section, and [`Containers`](ChunkDataContainer) for blocks and
/// biomes.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct Section {
    /// The number of non-air blocks in the section.
    pub block_count: usize,
    /// The block data stored in the section.
    pub blocks: ChunkDataContainer<BlockContainer>,
    /// The biome data stored in the section.
    pub biomes: ChunkDataContainer<BiomeContainer>,
}

#[allow(clippy::let_and_return)]
impl Section {
    /// The width of a [`Section`].
    ///
    /// This is the same as the width of a [`Chunk`].
    pub const WIDTH: usize = Chunk::WIDTH;

    /// The depth of a [`Section`].
    ///
    /// This is the same as the depth of a [`Chunk`].
    pub const DEPTH: usize = Chunk::DEPTH;

    /// The height of a [`Section`].
    ///
    /// This is always 16.
    pub const HEIGHT: usize = 16;

    /// The total volume of a [`Section`].
    pub const VOLUME: usize = Self::WIDTH * Self::DEPTH * Self::HEIGHT;

    /// Decodes a [`Section`] from a buffer.
    pub(crate) fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, ChunkDecodeError> {
        Ok(Self {
            block_count: usize::from(u16::fg_read(buf)?),
            blocks: ChunkDataContainer::decode(buf)?,
            biomes: ChunkDataContainer::decode(buf)?,
        })
    }

    /// Gets the block id at the given position in the section.
    #[must_use]
    pub const fn get_block(&self, pos: &SectionBlockPosition) -> usize { self.blocks.get(pos) }

    /// Sets the block id at the given position in the section.
    ///
    /// Returns the previous block id at the position.
    pub fn set_block(&mut self, pos: &SectionBlockPosition, value: usize) -> usize {
        self.blocks.set(pos, value)
    }

    /// Gets the biome id at the given position in the section.
    #[must_use]
    pub const fn get_biome(&self, pos: &SectionBlockPosition) -> usize { self.biomes.get(pos) }

    /// Sets the biome id at the given position in the section.
    ///
    /// Returns the previous biome id at the position.
    pub fn set_biome(&mut self, pos: &SectionBlockPosition, value: usize) -> usize {
        self.biomes.set(pos, value)
    }
}
