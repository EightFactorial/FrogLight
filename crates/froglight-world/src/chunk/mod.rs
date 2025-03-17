//! [`Chunk`], [`ChunkStorage`], [`Section`], and [`SectionData`].

#[cfg(feature = "block")]
use froglight_block::{prelude::*, resolve::BlockResolver};

mod palette;
pub use palette::SectionPalette;

mod section;
pub use section::{Section, SectionData};

mod storage;
pub use storage::{ArrayChunkStorage, ChunkStorage, VecChunkStorage};

use crate::prelude::BlockPos;

#[cfg(test)]
mod test;

/// A chunk of the world.
#[expect(dead_code)]
pub struct Chunk {
    #[cfg(feature = "nbt")]
    data: froglight_nbt::nbt::UnnamedNbt,
    storage: ChunkStorage,
}

impl Chunk {
    /// Create a new [`Chunk`] with the given [`ChunkStorage`].
    #[must_use]
    pub const fn new(storage: ChunkStorage) -> Self {
        Self {
            #[cfg(feature = "nbt")]
            data: froglight_nbt::nbt::UnnamedNbt::new_empty(),
            storage,
        }
    }

    /// Get a raw block id at the given [`BlockPos`].
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_raw_block(&self, _pos: BlockPos) -> Option<u32> { todo!() }

    /// Get a raw biome id at the given [`BlockPos`].
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_raw_biome(&self, _pos: BlockPos) -> Option<u32> { todo!() }
}

#[cfg(feature = "block")]
impl Chunk {
    /// Get the [`BlockType`] at the given [`BlockPos`].
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_block_trait<V: froglight_common::version::Version>(
        &self,
        _storage: &BlockStorage<V>,
    ) -> Option<&'static dyn BlockType<V>> {
        todo!()
    }

    /// Get the [`UntypedBlock`] at the given [`BlockPos`].
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_block_untyped<V: froglight_common::version::Version>(
        &self,
        _storage: &BlockStorage<V>,
    ) -> Option<UntypedBlock<V>> {
        todo!()
    }

    /// Get the [`Block`] at the given [`BlockPos`].
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_block_typed<V: froglight_common::version::Version, R: BlockResolver<V>>(
        &self,
        _storage: &BlockStorage<V>,
    ) -> Option<R::BlockEnum> {
        todo!()
    }
}

#[cfg(feature = "nbt")]
impl Chunk {
    /// Create a new [`Chunk`] with the given
    /// [`ChunkStorage`] and [`UnnamedNbt`](froglight_nbt::nbt::UnnamedNbt).
    #[must_use]
    #[cfg(feature = "nbt")]
    pub const fn new_with(storage: ChunkStorage, data: froglight_nbt::nbt::UnnamedNbt) -> Self {
        Self { data, storage }
    }

    /// Get the [`UnnamedNbt`](froglight_nbt::nbt::UnnamedNbt) of the [`Chunk`].
    #[inline]
    #[must_use]
    pub const fn chunk_data(&self) -> &froglight_nbt::nbt::UnnamedNbt { &self.data }

    /// Get the [`UnnamedNbt`](froglight_nbt::nbt::UnnamedNbt) of the [`Chunk`]
    /// mutably.
    #[inline]
    #[must_use]
    pub fn chunk_data_mut(&mut self) -> &mut froglight_nbt::nbt::UnnamedNbt { &mut self.data }

    /// Get the [`UnnamedNbt`](froglight_nbt::nbt::UnnamedNbt) of a [`BlockPos`]
    /// in the [`Chunk`].
    #[must_use]
    pub fn block_data(&self, _pos: BlockPos) -> Option<&froglight_nbt::nbt::UnnamedNbt> { todo!() }

    /// Get the [`UnnamedNbt`](froglight_nbt::nbt::UnnamedNbt) of a [`BlockPos`]
    /// in the [`Chunk`] mutably.
    #[must_use]
    pub fn block_data_mut(
        &mut self,
        _pos: BlockPos,
    ) -> Option<&mut froglight_nbt::nbt::UnnamedNbt> {
        todo!()
    }
}
