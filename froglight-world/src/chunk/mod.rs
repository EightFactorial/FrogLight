//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

use crate::{borrowed::storage::BorrowedChunkStorage, component::ChunkBlockPos, prelude::*};

#[cfg(feature = "froglight-block")]
mod block;

mod shared;
pub use shared::SharedChunk;

/// A region of blocks in a world.
#[derive(Default, Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Clone, Default, Component))]
pub struct Chunk {
    storage: BorrowedChunkStorage<'static>, // TODO: TEMPORARY
}

impl Chunk {
    /// Get the height of this [`Chunk`].
    #[must_use]
    pub const fn height(&self) -> u32 { todo!() }

    /// Get the height offset of this [`Chunk`].
    #[must_use]
    pub const fn height_offset(&self) -> i32 { todo!() }

    /// Get a reference to the sections in this [`Chunk`].
    #[must_use]
    pub const fn sections(&self) -> &[()] { todo!() }

    /// Get a mutable reference to the sections in this [`Chunk`].
    #[must_use]
    pub const fn sections_mut(&mut self) -> &mut [()] { todo!() }

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
