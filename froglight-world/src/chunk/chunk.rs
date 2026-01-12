//! TODO

use core::ops::Range;

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use froglight_biome::{biome::Biome, storage::GlobalBiomeStorage, version::BiomeVersion};
use froglight_block::{block::Block, storage::GlobalBlockStorage, version::BlockVersion};

use crate::{
    chunk::{Section, storage::ChunkStorage},
    component::ChunkBlockPos,
    prelude::BlockPos,
};

/// A region of blocks in a world.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Clone, Component))]
pub struct Chunk {
    biomes: &'static GlobalBiomeStorage,
    blocks: &'static GlobalBlockStorage,
    storage: ChunkStorage,
}

impl Chunk {
    /// Create a new [`Chunk`] using blocks and biomes from the given
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    pub const fn new<V: BiomeVersion + BlockVersion>(storage: ChunkStorage) -> Self {
        Self::new_from(storage, V::BIOMES, V::BLOCKS)
    }

    /// Create a new [`Chunk`] using the given block and biome storages.
    #[must_use]
    pub const fn new_from(
        storage: ChunkStorage,
        biomes: &'static GlobalBiomeStorage,
        blocks: &'static GlobalBlockStorage,
    ) -> Self {
        Self { biomes, blocks, storage }
    }

    /// Create a new empty large [`Chunk`].
    ///
    /// This is equivalent to an overworld chunk,
    /// or 24 sections (384 blocks) tall with an offset of -64.
    #[must_use]
    pub fn new_empty_large<V: BiomeVersion + BlockVersion>() -> Self {
        Self::new::<V>(ChunkStorage::empty_large())
    }

    /// Create a new empty normal [`Chunk`].
    ///
    /// This is equivalent to a nether or end chunk,
    /// or 16 sections (256 blocks) tall with an offset of 0.
    #[must_use]
    pub fn new_empty_normal<V: BiomeVersion + BlockVersion>() -> Self {
        Self::new::<V>(ChunkStorage::empty_normal())
    }

    /// Get the [`GlobalBlockStorage`] used by this chunk.
    #[inline]
    #[must_use]
    pub const fn biomes(&self) -> &'static GlobalBiomeStorage { self.biomes }

    /// Get the [`GlobalBlockStorage`] used by this chunk.
    #[inline]
    #[must_use]
    pub const fn blocks(&self) -> &'static GlobalBlockStorage { self.blocks }

    /// Get the height of this [`Chunk`].
    ///
    /// ## Note
    ///
    /// This is the height in world/coordinate space,
    /// and takes into account the chunk's vertical offset.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "Chunks will never be that tall")]
    #[expect(clippy::cast_possible_wrap, reason = "Chunks will never be that tall")]
    pub const fn height(&self) -> i32 {
        (self.storage.len() as i32 * 16).saturating_add(self.height_offset())
    }

    /// Get the height range of this [`Chunk`].
    ///
    /// ## Note
    ///
    /// This is the range in world/coordinate space and follows the chunk's
    /// vertical offset.
    #[must_use]
    pub const fn height_range(&self) -> Range<i32> { self.height_offset()..self.height() }

    /// Get the total height of this [`Chunk`], ignoring it's vertical offset.
    ///
    /// ## Note
    ///
    /// In other words, `y = 0` is always the bottom of the chunk.
    ///
    /// In most cases, you probably want [`Chunk::height`] instead.
    #[must_use]
    pub const fn height_total(&self) -> usize { self.storage.len() * 16 }

    /// Get the height offset of this [`Chunk`].
    #[must_use]
    pub const fn height_offset(&self) -> i32 { self.storage.offset() }

    /// Get a reference to the sections in this [`Chunk`].
    #[must_use]
    pub const fn sections(&self) -> &[Section] { self.storage.as_slice() }

    /// Get a mutable reference to the sections in this [`Chunk`].
    #[must_use]
    pub const fn sections_mut(&mut self) -> &mut [Section] { self.storage.as_slice_mut() }

    /// Get the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized.
    #[must_use]
    pub fn get_block<P: Into<BlockPos>>(&self, position: P) -> Option<Block> {
        ChunkBlockPos::try_from_blockpos(position.into(), self.height_offset())
            .and_then(|pos| self.get_block_pos::<ChunkBlockPos>(pos))
    }

    /// Get the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized.
    #[must_use]
    pub fn get_block_pos<P: Into<ChunkBlockPos>>(&self, position: P) -> Option<Block> {
        let position = position.into();
        let index = position.as_section_index();

        if let Some(section) = self.storage.as_slice().get(index) {
            use froglight_block::block::GlobalId;

            let raw = section.get_raw_block(position.as_section_blockpos());
            self.blocks.read().get_block(GlobalId::new(raw))
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_world", "Failed to access `Chunk`, position was invalid?");
            None
        }
    }

    /// Set the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized.
    #[must_use]
    pub fn set_block<P: Into<BlockPos>>(&mut self, position: P, block: Block) -> Option<Block> {
        ChunkBlockPos::try_from_blockpos(position.into(), self.height_offset())
            .and_then(|pos| self.set_block_pos::<ChunkBlockPos>(pos, block))
    }

    /// Set the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized.
    pub fn set_block_pos<P: Into<ChunkBlockPos>>(
        &mut self,
        position: P,
        block: Block,
    ) -> Option<Block> {
        let position = position.into();
        let index = position.as_section_index();

        if let Some(section) = self.storage.as_slice_mut().get_mut(index) {
            use froglight_block::block::GlobalId;

            let storage = self.blocks.read();

            if let Some(lookup) = storage.get_block(block.global_id())
                && lookup.block_ty() == block.block_ty()
            {
                let is_air =
                    |id| storage.get_block(GlobalId::new(id)).is_some_and(|block| block.is_air());

                let raw = section.set_raw_block(
                    position.as_section_blockpos(),
                    block.global_id().into_inner(),
                    is_air,
                );
                storage.get_block(GlobalId::new(raw))
            } else {
                #[cfg(feature = "tracing")]
                tracing::warn!(target: "froglight_world", "Failed to set `Block`, block did not match storage version?");
                None
            }
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_world", "Failed to access `Chunk`, position was invalid?");
            None
        }
    }

    /// Get the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized.
    #[must_use]
    pub fn get_biome<P: Into<BlockPos>>(&self, position: P) -> Option<Biome> {
        ChunkBlockPos::try_from_blockpos(position.into(), self.height_offset())
            .and_then(|pos| self.get_biome_pos::<ChunkBlockPos>(pos))
    }

    /// Get the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized.
    #[must_use]
    pub fn get_biome_pos<P: Into<ChunkBlockPos>>(&self, position: P) -> Option<Biome> {
        let position = position.into();
        let index = position.as_section_index();

        if let Some(section) = self.storage.as_slice().get(index) {
            use froglight_biome::biome::GlobalId;

            let raw = section.get_raw_biome(position.as_section_blockpos());
            self.biomes.read().get_biome(GlobalId::new(raw))
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_world", "Failed to access `Chunk`, position was invalid?");
            None
        }
    }

    /// Set the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized.
    #[must_use]
    pub fn set_biome<P: Into<BlockPos>>(&mut self, position: P, biome: Biome) -> Option<Biome> {
        ChunkBlockPos::try_from_blockpos(position.into(), self.height_offset())
            .and_then(|pos| self.set_biome_pos::<ChunkBlockPos>(pos, biome))
    }

    /// Set the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized.
    pub fn set_biome_pos<P: Into<ChunkBlockPos>>(
        &mut self,
        position: P,
        biome: Biome,
    ) -> Option<Biome> {
        let position = position.into();
        let index = position.as_section_index();

        if let Some(section) = self.storage.as_slice_mut().get_mut(index) {
            use froglight_biome::biome::GlobalId;

            let biomes = self.biomes.read();

            if let Some(lookup) = biomes.get_biome(biome.global_id())
                && lookup.biome_ty() == biome.biome_ty()
            {
                let raw = section
                    .set_raw_biome(position.as_section_blockpos(), biome.global_id().into_inner());
                biomes.get_biome(GlobalId::new(raw))
            } else {
                #[cfg(feature = "tracing")]
                tracing::warn!(target: "froglight_world", "Failed to set `Biome`, biome did not match storage version?");
                None
            }
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_world", "Failed to access `Chunk`, position was invalid?");
            None
        }
    }
}
