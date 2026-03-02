//! TODO

use core::ops::Range;

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use froglight_biome::{biome::Biome, storage::GlobalBiomeStorage, version::BiomeVersion};
use froglight_block::{block::Block, storage::GlobalBlockStorage, version::BlockVersion};
use smallvec::SmallVec;

use super::section::SectionPalette;
use crate::{
    chunk::{Section, storage::ChunkStorage},
    component::ChunkBlockPos,
    prelude::{BlockPos, NaiveChunk},
};

/// A region of blocks in a world.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Clone, Component))]
pub struct Chunk {
    biomes: &'static GlobalBiomeStorage,
    blocks: &'static GlobalBlockStorage,
    naive: NaiveChunk,
}

impl Chunk {
    /// Create a new [`Chunk`] using blocks and biomes from the given
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    pub fn new<V: BiomeVersion + BlockVersion>(naive: NaiveChunk) -> Self {
        Self { biomes: V::biomes(), blocks: V::blocks(), naive }
    }

    /// Create a new [`Chunk`] using the given [`ChunkStorage`].
    #[inline]
    #[must_use]
    pub const fn new_from(
        storage: ChunkStorage,
        biomes: &'static GlobalBiomeStorage,
        blocks: &'static GlobalBlockStorage,
    ) -> Self {
        Self { biomes, blocks, naive: NaiveChunk::new(storage) }
    }

    /// Create a new empty large [`Chunk`].
    ///
    /// This is equivalent to an overworld chunk,
    /// or 24 sections (384 blocks) tall with an offset of -64.
    #[must_use]
    pub fn new_empty_large<V: BiomeVersion + BlockVersion>() -> Self {
        Self::new_from(ChunkStorage::empty_large(), V::biomes(), V::blocks())
    }

    /// Create a new empty normal [`Chunk`].
    ///
    /// This is equivalent to a nether or end chunk,
    /// or 16 sections (256 blocks) tall with an offset of 0.
    #[must_use]
    pub fn new_empty_normal<V: BiomeVersion + BlockVersion>() -> Self {
        Self::new_from(ChunkStorage::empty_normal(), V::biomes(), V::blocks())
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
    pub const fn height(&self) -> i32 { self.naive.height() }

    /// Get the height range of this [`Chunk`].
    ///
    /// ## Note
    ///
    /// This is the range in world/coordinate space and follows the chunk's
    /// vertical offset.
    #[must_use]
    pub const fn height_range(&self) -> Range<i32> { self.naive.height_range() }

    /// Get the total height of this [`Chunk`], ignoring it's vertical offset.
    ///
    /// ## Note
    ///
    /// In other words, `y = 0` is always the bottom of the chunk.
    ///
    /// In most cases, you probably want [`Chunk::height`] instead.
    #[must_use]
    pub const fn height_total(&self) -> usize { self.naive.height_total() }

    /// Get the height offset of this [`Chunk`].
    #[must_use]
    pub const fn height_offset(&self) -> i32 { self.naive.height_offset() }

    /// Get a reference to the sections in this [`Chunk`].
    #[must_use]
    pub const fn sections(&self) -> &[Section] { self.naive.sections() }

    /// Get a mutable reference to the sections in this [`Chunk`].
    #[must_use]
    pub const fn sections_mut(&mut self) -> &mut [Section] { self.naive.sections_mut() }

    /// Get the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized.
    #[must_use]
    pub fn get_block<P: Into<BlockPos>>(&self, position: P) -> Option<Block> {
        self.naive.get_block_using::<P>(position, &self.blocks().load())
    }

    /// Get the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the block is not recognized.
    #[must_use]
    pub fn get_block_pos<P: Into<ChunkBlockPos>>(&self, position: P) -> Option<Block> {
        self.naive.get_block_pos_using::<P>(position, &self.blocks().load())
    }

    /// Set the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the [`Block`] is from another
    /// [`Version`](froglight_common::version::Version),
    /// position is out of bounds, or if the block is not recognized.
    pub fn set_block<P: Into<BlockPos>>(&mut self, position: P, block: Block) -> Option<Block> {
        if block.version_ty() != self.blocks.version_ty() {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_world", "Failed to set `Chunk` block, version mismatch");
            return None;
        }

        self.naive.set_block_using::<P>(position, block, &self.blocks().load())
    }

    /// Set the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the [`Block`] is from another
    /// [`Version`](froglight_common::version::Version),
    /// the position is out of bounds, or if the block is not recognized.
    pub fn set_block_pos<P: Into<ChunkBlockPos>>(
        &mut self,
        position: P,
        block: Block,
    ) -> Option<Block> {
        if block.version_ty() != self.blocks.version_ty() {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_world", "Failed to set `Chunk` block, version mismatch");
            return None;
        }

        self.naive.set_block_pos_using::<P>(position, block, &self.blocks().load())
    }

    /// Get the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized.
    #[must_use]
    pub fn get_biome<P: Into<BlockPos>>(&self, position: P) -> Option<Biome> {
        self.naive.get_biome_using::<P>(position, &self.biomes().load())
    }

    /// Get the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized.
    #[must_use]
    pub fn get_biome_pos<P: Into<ChunkBlockPos>>(&self, position: P) -> Option<Biome> {
        self.naive.get_biome_pos_using::<P>(position, &self.biomes().load())
    }

    /// Set the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the [`Biome`] is from another
    /// [`Version`](froglight_common::version::Version),
    /// the position is out of bounds, or if the biome is not recognized.
    pub fn set_biome<P: Into<BlockPos>>(&mut self, position: P, biome: Biome) -> Option<Biome> {
        if biome.version_ty() != self.biomes.version_ty() {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_world", "Failed to set `Chunk` biome, version mismatch");
            return None;
        }

        self.naive.set_biome_using::<P>(position, biome, &self.biomes().load())
    }

    /// Set the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the [`Biome`] is from another
    /// [`Version`](froglight_common::version::Version),
    /// the position is out of bounds, or if the biome is not recognized.
    pub fn set_biome_pos<P: Into<ChunkBlockPos>>(
        &mut self,
        position: P,
        biome: Biome,
    ) -> Option<Biome> {
        if biome.version_ty() != self.biomes.version_ty() {
            #[cfg(feature = "tracing")]
            tracing::warn!(target: "froglight_world", "Failed to set `Chunk` biome, version mismatch");
            return None;
        }

        self.naive.set_biome_pos_using::<P>(position, biome, &self.biomes.load())
    }

    /// Convert this [`Chunk`] into another version.
    ///
    /// Uses [`Air`](froglight_block::prelude::block::Air) for blocks and
    /// [`Plains`](froglight_biome::prelude::biome::Plains) for biomes that
    /// cannot be converted.
    #[must_use]
    #[expect(unused, reason = "WIP")]
    pub fn convert_into<V: BiomeVersion + BlockVersion>(&self) -> Self {
        // Skip if the chunk is already in the correct version.
        if self.biomes.version_ty() == V::biomes().version_ty()
            && self.blocks.version_ty() == V::blocks().version_ty()
        {
            return self.clone();
        }

        let new_biomes = V::biomes().load();
        let old_biomes = self.biomes.load();
        // let mut biome_cache = Vec::with_capacity(16);

        let biome_convert = |old: u32| -> u32 { todo!() };

        let new_blocks = V::blocks().load();
        let old_blocks = self.blocks.load();
        // let mut block_cache = Vec::with_capacity(16);

        let block_convert = |old: u32| -> u32 { todo!() };

        let mut chunk = self.clone();

        for section in chunk.sections_mut() {
            let biome = section.biome_data_mut();
            match biome.palette() {
                SectionPalette::Single(old) => {
                    // SAFETY: Only the palette is being modified
                    *unsafe { biome.palette_mut() } = SectionPalette::Single(biome_convert(*old));
                }
                SectionPalette::Vector(vals) => {
                    let mut new = SmallVec::with_capacity(vals.len());
                    for &old in vals {
                        new.push(biome_convert(old));
                    }
                    // SAFETY: Only the palette is being modified
                    *unsafe { biome.palette_mut() } = SectionPalette::Vector(new);
                }
                SectionPalette::Global => todo!("Set each biome individually"),
            }

            let block = section.block_data_mut();
            match block.palette() {
                SectionPalette::Single(old) => {
                    // SAFETY: Only the palette is being modified
                    *unsafe { block.palette_mut() } = SectionPalette::Single(block_convert(*old));
                }
                SectionPalette::Vector(vals) => {
                    let mut new = SmallVec::with_capacity(vals.len());
                    for &old in vals {
                        new.push(block_convert(old));
                    }
                    // SAFETY: Only the palette is being modified
                    *unsafe { block.palette_mut() } = SectionPalette::Vector(new);
                }
                SectionPalette::Global => todo!("Set each block individually"),
            }
        }

        // Use the new version's biome and block storage.
        chunk.biomes = V::biomes();
        chunk.blocks = V::blocks();

        chunk
    }
}
