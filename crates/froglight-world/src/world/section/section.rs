//! A [`Section`] is a 16x16x16 section of a [`Chunk`](crate::world::Chunk).

use std::io::Cursor;

use bevy_reflect::Reflect;
use froglight_core::data::SectionBlockPosition;
use froglight_protocol::{io::FrogRead, traits::Version};

use super::{SectionBlockIterator, SectionIdIterator};
use crate::world::{
    chunk::ChunkDecodeError, container::ContainerType, BiomeContainer, BlockContainer, Chunk,
    ChunkDataContainer,
};

/// A [`Section`] is a `16x16x16 (X,Y,Z)` section of a
/// [`Chunk`](crate::world::Chunk).
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
    #[inline]
    pub fn get_blockid(&self, pos: SectionBlockPosition) -> usize { Self::get(&self.blocks, pos) }

    /// Sets the block id at the given position in the section.
    ///
    /// Returns the previous block id at the position.
    #[inline]
    pub fn set_blockid(&mut self, pos: SectionBlockPosition, value: usize) -> usize {
        Self::set(&mut self.blocks, pos, value)
    }

    /// Gets the biome id at the given position in the section.
    #[must_use]
    #[inline]
    pub fn get_biomeid(&self, pos: SectionBlockPosition) -> usize { Self::get(&self.biomes, pos) }

    /// Sets the biome id at the given position in the section.
    ///
    /// Returns the previous biome id at the position.
    #[inline]
    pub fn set_biomeid(&mut self, pos: SectionBlockPosition, value: usize) -> usize {
        Self::set(&mut self.biomes, pos, value)
    }

    #[inline]
    fn get<T: ContainerType>(
        container: &ChunkDataContainer<T>,
        pos: SectionBlockPosition,
    ) -> usize {
        container.get_data(pos)
    }

    #[must_use]
    #[inline]
    fn set<T: ContainerType>(
        container: &mut ChunkDataContainer<T>,
        pos: SectionBlockPosition,
        value: usize,
    ) -> usize {
        container.set_data(pos, value)
    }

    /// Creates a new [`SectionIdIterator`] for the section.
    ///
    /// This starts at the first block in the section,
    /// and iterates over all of the blocks in the section.
    #[must_use]
    #[inline]
    pub const fn blockid_iter(&self) -> SectionIdIterator<'_> { SectionIdIterator::new(self) }

    /// Creates a new [`SectionBlockIterator`] for the section.
    ///
    /// This starts at the first block in the section,
    /// and iterates over all of the blocks in the section.
    #[must_use]
    #[inline]
    pub const fn block_iter<V: Version>(&self) -> SectionBlockIterator<'_, V> {
        SectionBlockIterator::new(self)
    }
}
