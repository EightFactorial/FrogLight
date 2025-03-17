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
    pub fn get_raw_block(&self, pos: BlockPos) -> Option<u32> { self.storage.get_raw_block(pos) }

    /// Set a raw block id at the given [`BlockPos`].
    ///
    /// Returns the previous block id,
    /// or `None` if the position is out of bounds.
    pub fn set_raw_block(
        &mut self,
        pos: BlockPos,
        block_id: u32,
        is_air: impl Fn(u32) -> bool,
    ) -> Option<u32> {
        self.storage.set_raw_block(pos, block_id, is_air)
    }

    /// Get a raw biome id at the given [`BlockPos`].
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_raw_biome(&self, pos: BlockPos) -> Option<u32> { self.storage.get_raw_biome(pos) }
}

#[cfg(feature = "block")]
impl Chunk {
    /// Get the [`BlockType`] at the given [`BlockPos`].
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_block_trait<V: froglight_common::version::Version>(
        &self,
        pos: BlockPos,
        storage: &BlockStorage<V>,
    ) -> Option<&'static dyn BlockType<V>> {
        self.get_raw_block(pos).and_then(|id| storage.get_trait(GlobalBlockId::new_unchecked(id)))
    }

    /// Get the [`UntypedBlock`] at the given [`BlockPos`].
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_block_untyped<V: froglight_common::version::Version>(
        &self,
        pos: BlockPos,
        storage: &BlockStorage<V>,
    ) -> Option<UntypedBlock<V>> {
        self.get_raw_block(pos).and_then(|id| storage.get_untyped(GlobalBlockId::new_unchecked(id)))
    }

    /// Get the [`Block`] at the given [`BlockPos`].
    ///
    /// Returns `None` if the position is out of bounds.
    #[must_use]
    pub fn get_block_typed<V: froglight_common::version::Version, R: BlockResolver<V>>(
        &self,
        pos: BlockPos,
        storage: &BlockStorage<V>,
    ) -> Option<R::BlockEnum> {
        self.get_raw_block(pos)
            .and_then(|id| storage.get_typed::<R>(GlobalBlockId::new_unchecked(id)))
    }

    /// Set a [`Block`] at the given [`BlockPos`].
    ///
    /// Returns the previous block id,
    /// or `None` if the position is out of bounds.
    pub fn set_block<V: froglight_common::version::Version>(
        &mut self,
        pos: BlockPos,
        block: impl Into<UntypedBlock<V>>,
        storage: &BlockStorage<V>,
    ) -> Option<u32> {
        self.set_raw_block(pos, storage.get_global(block)?.into(), |id: u32| {
            storage.get_trait(GlobalBlockId::new_unchecked(id)).is_some_and(BlockType::is_air)
        })
    }
}

#[cfg(feature = "nbt")]
impl Chunk {
    /// Create a new [`Chunk`] with the given
    /// [`ChunkStorage`] and [`UnnamedNbt`](froglight_nbt::nbt::UnnamedNbt).
    #[must_use]
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
    pub const fn chunk_data_mut(&mut self) -> &mut froglight_nbt::nbt::UnnamedNbt { &mut self.data }

    /// Get the [`UnnamedNbt`](froglight_nbt::nbt::UnnamedNbt) of a [`Section`].
    #[must_use]
    pub fn section_data(
        &self,
        pos: BlockPos,
    ) -> Option<&hashbrown::HashMap<crate::position::SectionBlockPos, froglight_nbt::nbt::UnnamedNbt>>
    {
        self.storage.get(pos).map(Section::block_data)
    }

    /// Get the [`UnnamedNbt`](froglight_nbt::nbt::UnnamedNbt) of a [`Section`]
    /// mutably.
    #[must_use]
    pub fn section_data_mut(
        &mut self,
        pos: BlockPos,
    ) -> Option<
        &mut hashbrown::HashMap<crate::position::SectionBlockPos, froglight_nbt::nbt::UnnamedNbt>,
    > {
        self.storage.get_mut(pos).map(Section::block_data_mut)
    }

    /// Get the [`UnnamedNbt`](froglight_nbt::nbt::UnnamedNbt) of a [`BlockPos`]
    /// in the [`Chunk`].
    #[must_use]
    pub fn block_data(&self, pos: BlockPos) -> Option<&froglight_nbt::nbt::UnnamedNbt> {
        self.section_data(pos)
            .and_then(|section| section.get(&crate::position::SectionBlockPos::from_block(pos)))
    }

    /// Get the [`UnnamedNbt`](froglight_nbt::nbt::UnnamedNbt) of a [`BlockPos`]
    /// in the [`Chunk`] mutably.
    #[must_use]
    pub fn block_data_mut(&mut self, pos: BlockPos) -> Option<&mut froglight_nbt::nbt::UnnamedNbt> {
        self.section_data_mut(pos)
            .and_then(|section| section.get_mut(&crate::position::SectionBlockPos::from_block(pos)))
    }
}
