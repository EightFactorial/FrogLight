//! Additional methods that require the [`froglight_block`] crate.

use froglight_block::{prelude::*, storage::BlockStorage};

use crate::{component::ChunkBlockPos, prelude::*};

impl Chunk {
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
}
