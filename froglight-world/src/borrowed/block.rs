//! Additional methods that require the [`froglight_block`] crate.

use core::any::TypeId;

#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
use froglight_block::block::BlockType;
use froglight_block::{block::GlobalId, prelude::*, storage::BlockStorage};

use crate::{
    borrowed::{BorrowedChunk, section::BorrowedPalette},
    component::ChunkBlockPos,
    prelude::*,
};

impl BorrowedChunk<'_> {
    /// Get the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized by the
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    pub fn get_block<V: BlockVersion, P: Into<BlockPos>>(&self, position: P) -> Option<Block> {
        self.get_block_using::<P>(position, &V::blocks().read())
    }

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

    /// Get the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized by the
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    pub fn get_block_pos<V: BlockVersion, P: Into<ChunkBlockPos>>(
        &self,
        position: P,
    ) -> Option<Block> {
        self.get_block_pos_using::<P>(position, &V::blocks().read())
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
        self.get_raw_block_pos::<P>(position).and_then(|id| storage.get_block(GlobalId::new(id)))
    }

    /// Returns `true` if the chunk contains at least one block of the same
    /// type.
    #[must_use]
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    pub fn contains_block<V: BlockVersion>(&self, block: Block) -> bool {
        self.contains_block_using(block, &V::blocks().read())
    }

    /// Returns `true` if the chunk contains at least one block of the same
    /// type.
    ///
    /// Resolves block types using the provided [`BlockStorage`].
    #[must_use]
    pub fn contains_block_using(&self, block: Block, storage: &BlockStorage) -> bool {
        self.contains_block_type_using(block.block_ty(), storage)
    }

    /// Returns `true` if the chunk contains at least one block of the exact
    /// same type and state.
    #[must_use]
    pub fn contains_block_exact(&self, block: Block) -> bool {
        self.contains_raw_block(block.global_id().into_inner())
    }

    /// Returns `true` if the chunk contains at least one block of the same
    /// type.
    #[must_use]
    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    pub fn contains_block_type<B: BlockType<V>, V: BlockVersion>(&self) -> bool {
        self.contains_block_type_using(B::METADATA.block_ty(), &V::blocks().read())
    }

    /// Returns `true` if the chunk contains at least one block of the same
    /// type.
    #[must_use]
    pub fn contains_block_type_using(&self, block_type: TypeId, storage: &BlockStorage) -> bool {
        // Closure to check if a block id matches the desired block type.
        let matches = |id: u32| {
            storage.get_block(GlobalId::new(id)).is_some_and(|block| block.block_ty() == block_type)
        };

        self.storage.as_slice().iter().any(|section| match section.block_data().palette() {
            BorrowedPalette::Single(id) => matches(*id),
            BorrowedPalette::Vector(vec) => vec.iter().any(|palette_id| {
                if matches(*palette_id) {
                    // Cannot return `true` directly as the palette may contain unused values.
                    section.iter_raw_blocks().any(matches)
                } else {
                    false
                }
            }),
            BorrowedPalette::Global => section.iter_raw_blocks().any(matches),
        })
    }
}
