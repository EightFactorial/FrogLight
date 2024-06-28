use froglight_protocol::{common::SectionBlockPosition, protocol::FrogReadWrite};

use crate::{BiomeContainer, BlockContainer, Chunk, SectionBlockIter};

/// A chunk section.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct ChunkSection {
    /// The number of non-air blocks in the section.
    pub block_count: u16,

    /// The block storage.
    pub blocks: BlockContainer,
    /// The biome storage.
    pub biomes: BiomeContainer,
}

// TODO: Increment/decrement `ChunkSection::block_count` when setting blocks.
impl ChunkSection {
    /// The width of a [`ChunkSection`].
    ///
    /// This is the same as the width of a [`Chunk`].
    pub const WIDTH: u32 = Chunk::WIDTH;

    /// The depth of a [`ChunkSection`].
    ///
    /// This is the same as the depth of a [`Chunk`].
    pub const DEPTH: u32 = Chunk::DEPTH;

    /// The height of a [`ChunkSection`].
    ///
    /// This is always 16.
    pub const HEIGHT: u32 = 16;

    /// The total volume of a [`ChunkSection`].
    pub const VOLUME: u32 = Self::WIDTH * Self::DEPTH * Self::HEIGHT;

    /// Returns an iterator over the blocks of the [`ChunkSection`].
    #[must_use]
    pub fn block_iter(&self) -> SectionBlockIter<'_> { SectionBlockIter::new(self) }

    /// Returns the `block state id` at the given position.
    #[inline]
    #[must_use]
    pub fn get_blockstate(&self, position: SectionBlockPosition) -> u32 {
        self.blocks.get_data(&position)
    }

    /// Sets the `block state id` at the given position.
    ///
    /// Returns the previous `block state id` at the position.
    #[inline]
    pub fn set_blockstate(&mut self, position: SectionBlockPosition, blockstate_id: u32) -> u32 {
        self.blocks.set_data(&position, blockstate_id)
    }

    /// Returns the `biome id` at the given position.
    #[inline]
    #[must_use]
    pub fn get_biome(&self, position: SectionBlockPosition) -> u32 {
        self.biomes.get_data(&position)
    }

    /// Sets the `biome id` at the given position.
    ///
    /// Returns the previous `biome id` at the position.
    #[inline]
    pub fn set_biome(&mut self, position: SectionBlockPosition, biome_id: u32) -> u32 {
        self.biomes.set_data(&position, biome_id)
    }
}
