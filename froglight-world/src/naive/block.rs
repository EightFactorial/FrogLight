//! Additional methods that require the [`froglight_block`] crate.

use core::any::TypeId;

use froglight_block::{prelude::*, storage::BlockStorage};

use crate::{component::ChunkBlockPos, naive::NaiveChunk, prelude::*, section::SectionPalette};

impl NaiveChunk {
    /// Get the [`Block`] at the given position within the chunk,
    /// resolving it using the provided [`BlockStorage`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized by the [`BlockStorage`].
    #[must_use]
    pub fn get_block_using<P: Into<BlockPos>>(
        &self,
        position: P,
        storage: &BlockStorage,
    ) -> Option<Block> {
        ChunkBlockPos::try_from_blockpos(position.into(), self.height_offset())
            .and_then(|pos| self.get_block_pos_using::<ChunkBlockPos>(pos, storage))
    }

    /// Get the [`Block`] at the given position within the chunk,
    /// resolving it using the provided [`BlockStorage`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized by the [`BlockStorage`].
    #[must_use]
    pub fn get_block_pos_using<P: Into<ChunkBlockPos>>(
        &self,
        position: P,
        storage: &BlockStorage,
    ) -> Option<Block> {
        self.get_raw_block_pos::<P>(position)
            .and_then(|id| storage.get_block_by_state(GlobalStateId::new(id)))
    }

    /// Set the [`Block`] at the given position within the chunk and return the
    /// previous one, resolving it using the provided [`BlockStorage`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized by the [`BlockStorage`].
    pub fn set_block_using<P: Into<BlockPos>>(
        &mut self,
        position: P,
        block: Block,
        storage: &BlockStorage,
    ) -> Option<Block> {
        ChunkBlockPos::try_from_blockpos(position.into(), self.height_offset())
            .and_then(|pos| self.set_block_pos_using::<ChunkBlockPos>(pos, block, storage))
    }

    /// Set the [`Block`] at the given position within the chunk and return the
    /// previous one, resolving it using the provided [`BlockStorage`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized by the [`BlockStorage`].
    pub fn set_block_pos_using<P: Into<ChunkBlockPos>>(
        &mut self,
        position: P,
        block: Block,
        storage: &BlockStorage,
    ) -> Option<Block> {
        let is_air = |id| {
            storage.get_block_by_state(GlobalStateId::new(id)).is_some_and(|block| block.is_air())
        };
        let is_fluid = |id| {
            storage
                .get_block_by_state(GlobalStateId::new(id))
                .is_some_and(|block| block.is_liquid())
        };

        let block_id = block.using_version_storage(storage)?.global_id().into_inner();
        self.set_raw_block_pos::<P>(position, block_id, is_air, is_fluid)
            .and_then(|id| storage.get_block_by_state(GlobalStateId::new(id)))
    }

    /// Returns `true` if the chunk contains at least one block of the exact
    /// same type and state.
    #[must_use]
    pub fn contains_block_exact(&self, block: Block) -> bool {
        self.contains_raw_block(block.global_id().into_inner())
    }

    /// Returns `true` if the chunk contains at least one block of the same
    /// type.
    ///
    /// Resolves block types using the provided [`BlockStorage`].
    #[must_use]
    pub fn contains_block_type(&self, block_type: TypeId, storage: &BlockStorage) -> bool {
        // Closure to check if a block id matches the desired block type.
        let matches = |id: u32| {
            storage
                .get_block_by_state(GlobalStateId::new(id))
                .is_some_and(|block| block.block_ty() == block_type)
        };

        self.storage.as_slice().iter().any(|section| match section.block_data().palette() {
            SectionPalette::Single(id) => matches(*id),
            SectionPalette::Vector(vec) => vec.iter().any(|palette_id| {
                if matches(*palette_id) {
                    // Cannot return `true` directly as the palette may contain unused values.
                    section.iter_raw_blocks().any(matches)
                } else {
                    false
                }
            }),
            SectionPalette::Global => section.iter_raw_blocks().any(matches),
        })
    }
}
