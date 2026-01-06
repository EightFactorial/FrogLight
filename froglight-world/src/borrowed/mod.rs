//! Chunks where data is borrowed from another source.
//!
//! Compatible with `no-alloc` environments.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

use crate::{component::ChunkBlockPos, prelude::*};

#[cfg(feature = "froglight-block")]
mod block;

pub mod section;
pub use section::BorrowedSection;

pub mod storage;
use storage::BorrowedChunkStorage;

/// A borrowed region of blocks in a world.
#[derive(Default, Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Clone, Default, Component))]
pub struct BorrowedChunk<'a> {
    storage: BorrowedChunkStorage<'a>,
}

impl<'a> BorrowedChunk<'a> {
    /// Create a new [`BorrowedChunk`] from the given storage.
    #[must_use]
    pub const fn new(storage: BorrowedChunkStorage<'a>) -> Self { Self { storage } }

    /// Create a new [`BorrowedChunk`] from the given sections and offset.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn new_from(sections: Vec<BorrowedSection<'a>>, offset: i32) -> Self {
        Self { storage: BorrowedChunkStorage::new_from_vec(sections, offset) }
    }

    /// Create a new empty large [`BorrowedChunk`].
    ///
    /// This is equivalent to an overworld chunk,
    /// or 24 sections (384 blocks) tall with an offset of -64.
    #[must_use]
    pub fn new_empty_large() -> Self { Self { storage: BorrowedChunkStorage::empty_large() } }

    /// Create a new empty normal [`BorrowedChunk`].
    ///
    /// This is equivalent to a nether or end chunk,
    /// or 16 sections (256 blocks) tall with an offset of 0.
    #[must_use]
    pub fn new_empty_normal() -> Self { Self { storage: BorrowedChunkStorage::empty_normal() } }

    /// Get the height of this [`Chunk`].
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "Chunks will never be that tall")]
    pub fn height(&self) -> u32 { self.storage.len() as u32 * 16 }

    /// Get the height offset of this [`Chunk`].
    #[must_use]
    pub const fn height_offset(&self) -> i32 { self.storage.offset() }

    /// Get a reference to the sections in this [`Chunk`].
    #[must_use]
    pub fn sections(&self) -> &[BorrowedSection<'a>] { self.storage.as_slice() }

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
}
