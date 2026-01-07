//! TODO

use core::ops::Range;

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

use crate::{component::ChunkBlockPos, prelude::*};

#[cfg(feature = "froglight-block")]
mod block;

pub mod section;
pub use section::Section;

mod shared;
pub use shared::SharedChunk;

pub mod storage;
use storage::ChunkStorage;

/// A region of blocks in a world.
#[derive(Default, Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Clone, Default, Component))]
pub struct Chunk {
    storage: ChunkStorage,
}

impl Chunk {
    /// Create a new [`Chunk`] from the given storage.
    #[must_use]
    pub const fn new(storage: ChunkStorage) -> Self { Self { storage } }

    // /// Create a new [`Chunk`] from the given sections and offset.
    // #[must_use]
    // pub fn new_from(sections: Vec<Section>, offset: i32) -> Self {
    //     Self { storage: ChunkStorage::new_from_vec(sections, offset) }
    // }

    /// Create a new empty large [`BorrowedChunk`].
    ///
    /// This is equivalent to an overworld chunk,
    /// or 24 sections (384 blocks) tall with an offset of -64.
    #[must_use]
    pub fn new_empty_large() -> Self { Self { storage: ChunkStorage::empty_large() } }

    /// Create a new empty normal [`BorrowedChunk`].
    ///
    /// This is equivalent to a nether or end chunk,
    /// or 16 sections (256 blocks) tall with an offset of 0.
    #[must_use]
    pub fn new_empty_normal() -> Self { Self { storage: ChunkStorage::empty_normal() } }

    /// Get the height of this [`Chunk`].
    ///
    /// ## Note
    ///
    /// This is the height in world/coordinate space,
    /// and takes into account the chunk's vertical offset.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "Chunks will never be that tall")]
    #[expect(clippy::cast_possible_wrap, reason = "Chunks will never be that tall")]
    pub fn height(&self) -> i32 {
        (self.storage.len() as i32 * 16).saturating_add(self.height_offset())
    }

    /// Get the height range of this [`Chunk`].
    ///
    /// ## Note
    ///
    /// This is the range in world/coordinate space and follows the chunk's
    /// vertical offset.
    #[must_use]
    pub fn height_range(&self) -> Range<i32> { self.height_offset()..self.height() }

    /// Get the total height of this [`Chunk`], ignoring it's vertical offset.
    ///
    /// ## Note
    ///
    /// In other words, `y = 0` is always the bottom of the chunk.
    ///
    /// In most cases, you probably want [`Chunk::height`] instead.
    #[must_use]
    pub fn height_total(&self) -> usize { self.storage.len() * 16 }

    /// Get the height offset of this [`Chunk`].
    #[must_use]
    pub const fn height_offset(&self) -> i32 { self.storage.offset() }

    /// Get a reference to the sections in this [`Chunk`].
    #[must_use]
    pub fn sections(&self) -> &[Section] { todo!() }

    /// Get a mutable reference to the sections in this [`Chunk`].
    #[must_use]
    pub fn sections_mut(&self) -> &mut [Section] { todo!() }

    /// Get the block id at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_raw_block<P: Into<BlockPos>>(&self, position: P) -> Option<u32> {
        ChunkBlockPos::try_from_blockpos(position.into(), self.height_offset())
            .and_then(|pos| self.get_raw_block_pos::<ChunkBlockPos>(pos))
    }

    /// Get the block id at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    #[allow(clippy::manual_map, reason = "Nuh-uh")]
    pub fn get_raw_block_pos<P: Into<ChunkBlockPos>>(&self, position: P) -> Option<u32> {
        let position = position.into();
        let index = position.as_section_index();

        if let Some(section) = self.storage.as_slice().get(index) {
            Some(section.get_raw_block(position.as_section_blockpos()))
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_world", "Failed to access `BorrowedChunk`, position was invalid?");
            None
        }
    }

    /// Iterate over all raw block ids in this chunk.
    ///
    /// # Note
    ///
    /// If you are searching for whether a block exists in the chunk, use
    /// [`Chunk::contains_raw_block`] instead as it has much better
    /// performance.
    pub fn iter_raw_blocks(&self) -> impl Iterator<Item = u32> + '_ {
        self.storage.as_slice().iter().flat_map(Section::iter_raw_blocks)
    }

    /// Returns `true` if the chunk contains the given raw block id.
    #[must_use]
    pub fn contains_raw_block(&self, block_id: u32) -> bool {
        self.storage.as_slice().iter().any(|section| section.contains_raw_block(block_id))
    }

    /// Get the biome id at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_raw_biome<P: Into<BlockPos>>(&self, position: P) -> Option<u32> {
        ChunkBlockPos::try_from_blockpos(position.into(), self.height_offset())
            .and_then(|pos| self.get_raw_biome_pos::<ChunkBlockPos>(pos))
    }

    /// Get the biome id at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    #[allow(clippy::manual_map, reason = "Nuh-uh")]
    pub fn get_raw_biome_pos<P: Into<ChunkBlockPos>>(&self, position: P) -> Option<u32> {
        let position = position.into();
        let index = position.as_section_index();

        if let Some(section) = self.storage.as_slice().get(index) {
            Some(section.get_raw_biome(position.as_section_blockpos()))
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_world", "Failed to access `BorrowedChunk`, position was invalid?");
            None
        }
    }

    /// Iterate over all raw biome ids in this chunk.
    ///
    /// # Note
    ///
    /// If you are searching for whether a biome exists in the chunk, use
    /// [`Chunk::contains_raw_biome`] instead as it has much better
    /// performance.
    pub fn iter_raw_biomes(&self) -> impl Iterator<Item = u32> + '_ {
        self.storage.as_slice().iter().flat_map(Section::iter_raw_biomes)
    }

    /// Returns `true` if the chunk contains the given raw biome id.
    #[must_use]
    pub fn contains_raw_biome(&self, biome_id: u32) -> bool {
        self.storage.as_slice().iter().any(|section| section.contains_raw_biome(biome_id))
    }
}
