//! [`Chunk`], [`ChunkStorage`], [`Section`], and [`SectionData`].
#![allow(clippy::struct_field_names)]

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "block")]
use froglight_block::{prelude::*, resolve::BlockResolver};
#[cfg(feature = "io")]
use froglight_io::prelude::*;
#[cfg(feature = "nbt")]
use froglight_nbt::nbt::UnnamedNbt;
#[cfg(feature = "nbt")]
use hashbrown::hash_map::{DefaultHashBuilder, Entry, HashMap};

#[cfg(feature = "nbt")]
mod entity;
#[cfg(feature = "nbt")]
pub use entity::PackedEntity;

mod palette;
pub use palette::SectionPalette;

mod section;
pub use section::{Section, SectionData};

mod storage;
pub use storage::{ArrayChunkStorage, ChunkStorage, VecChunkStorage};

#[cfg(feature = "nbt")]
use crate::position::RelativeBlockPos;
use crate::prelude::BlockPos;

#[cfg(test)]
mod test;

/// A chunk of the world.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect), reflect(Component))]
pub struct Chunk {
    storage: ChunkStorage,
    #[cfg(feature = "nbt")]
    chunk_data: UnnamedNbt,
    #[cfg(feature = "nbt")]
    block_data: HashMap<RelativeBlockPos, PackedEntity>,
}

impl Chunk {
    /// Create a new [`Chunk`] with the given [`ChunkStorage`].
    #[must_use]
    pub fn new(storage: ChunkStorage) -> Self {
        Self {
            storage,
            #[cfg(feature = "nbt")]
            chunk_data: UnnamedNbt::new_empty(),
            #[cfg(feature = "nbt")]
            block_data: HashMap::new(),
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
    ///
    /// # Note
    /// `is_air` is a function that returns `true` if the block id is air.
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

    /// Set a raw biome id at the given [`BlockPos`].
    ///
    /// Returns the previous biome id,
    /// or `None` if the position is out of bounds.
    pub fn set_raw_biome(&mut self, pos: BlockPos, biome_id: u32) -> Option<u32> {
        self.storage.set_raw_biome(pos, biome_id)
    }
}

// -------------------------------------------------------------------------------------------------

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

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "nbt")]
impl Chunk {
    /// Create a new [`Chunk`] with the given
    /// [`ChunkStorage`] and [`UnnamedNbt`].
    #[must_use]
    pub const fn new_with(
        storage: ChunkStorage,
        chunk_data: UnnamedNbt,
        block_data: HashMap<RelativeBlockPos, PackedEntity>,
    ) -> Self {
        Self { storage, chunk_data, block_data }
    }

    /// Get the [`UnnamedNbt`] of the [`Chunk`].
    #[inline]
    #[must_use]
    pub const fn chunk_data(&self) -> &UnnamedNbt { &self.chunk_data }

    /// Get the [`UnnamedNbt`] of the [`Chunk`]
    /// mutably.
    #[inline]
    #[must_use]
    pub const fn chunk_data_mut(&mut self) -> &mut UnnamedNbt { &mut self.chunk_data }

    /// Get the [`UnnamedNbt`] of a [`BlockPos`].
    #[inline]
    #[must_use]
    pub fn block_data(&self, pos: BlockPos) -> Option<&UnnamedNbt> {
        self.block_data
            .get(&RelativeBlockPos::from_block(pos, self.storage.height_min()))
            .map(|entity| &entity.entity_data)
    }

    /// Get the [`UnnamedNbt`] of a [`BlockPos`] mutably.
    #[inline]
    #[must_use]
    pub fn block_data_mut(&mut self, pos: BlockPos) -> Option<&mut UnnamedNbt> {
        self.block_data
            .get_mut(&RelativeBlockPos::from_block(pos, self.storage.height_min()))
            .map(|entity| &mut entity.entity_data)
    }

    /// Get a [`BlockPos`]'s NBT data entry.
    #[inline]
    #[must_use]
    pub fn block_data_entry(
        &mut self,
        pos: BlockPos,
    ) -> Entry<'_, RelativeBlockPos, PackedEntity, DefaultHashBuilder> {
        self.block_data.entry(RelativeBlockPos::from_block(pos, self.storage.height_min()))
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl Chunk {
    /// Read the data from the given buffer.
    ///
    /// # Errors
    /// Returns an error if the type does not match the buffer,
    /// or if the buffer fails to be read from.
    pub fn frog_read(offset: isize, buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        let chunk_data = UnnamedNbt::frog_read(buffer)?;
        let sections = Vec::<Section>::frog_read(buffer)?;
        let block_data = Vec::<PackedEntity>::frog_read(buffer)?;
        Ok(Self {
            chunk_data,
            block_data: PackedEntity::list_into_map(block_data),
            storage: ChunkStorage::from_sections(sections, offset),
        })
    }
}

#[cfg(feature = "io")]
impl FrogWrite for Chunk {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        let mut written = 0;
        written += self.chunk_data.frog_write(buffer)?;
        written += self.storage.sections_ref().frog_write(buffer)?;
        written += self
            .block_data
            .values()
            .try_fold(0, |acc, entity| entity.frog_write(buffer).map(|w| acc + w))?;
        Ok(written)
    }

    fn frog_len(&self) -> usize {
        self.chunk_data.frog_len()
            + self.storage.sections_ref().frog_len()
            + self.block_data.values().map(PackedEntity::frog_len).sum::<usize>()
    }
}
