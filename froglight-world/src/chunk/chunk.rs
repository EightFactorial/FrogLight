//! TODO

use core::{any::TypeId, fmt, ops::Range};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use froglight_biome::{
    biome::Biome, prelude::GlobalBiomeId, storage::BiomeStorage, version::BiomeVersion,
};
use froglight_block::{
    block::Block, prelude::GlobalStateId, storage::BlockStorage, version::BlockVersion,
};
use froglight_common::prelude::Identifier;
use smallvec::SmallVec;

use crate::{
    component::ChunkBlockPos,
    naive::storage::ChunkStorage,
    prelude::{BlockPos, NaiveChunk},
    section::{BiomeSection, Section, SectionPalette, SectionType},
};

/// A region of blocks in a world.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Clone, Component))]
#[cfg_attr(feature = "facet", derive(facet::Facet), facet(opaque))]
pub struct Chunk {
    biomes: &'static BiomeStorage,
    blocks: &'static BlockStorage,
    naive: NaiveChunk,
}

impl Chunk {
    /// Create a new [`Chunk`] using blocks and biomes from the given
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    pub fn new<V: BiomeVersion + BlockVersion>(naive: NaiveChunk) -> Self {
        Self { biomes: V::biomes(), blocks: V::blocks(), naive }
    }

    /// Create a new empty large [`Chunk`].
    ///
    /// This is equivalent to an overworld chunk,
    /// or 24 sections (384 blocks) tall with an offset of -64.
    #[must_use]
    pub fn empty_large<V: BiomeVersion + BlockVersion>() -> Self {
        Self::new::<V>(NaiveChunk::new(ChunkStorage::empty_large()))
    }

    /// Create a new empty normal [`Chunk`].
    ///
    /// This is equivalent to a nether or end chunk,
    /// or 16 sections (256 blocks) tall with an offset of 0.
    #[must_use]
    pub fn empty_normal<V: BiomeVersion + BlockVersion>() -> Self {
        Self::new::<V>(NaiveChunk::new(ChunkStorage::empty_normal()))
    }

    /// Get the [`BiomeStorage`] used by this chunk.
    #[inline]
    #[must_use]
    pub const fn biomes(&self) -> &'static BiomeStorage { self.biomes }

    /// Get the [`BlockStorage`] used by this chunk.
    #[inline]
    #[must_use]
    pub const fn blocks(&self) -> &'static BlockStorage { self.blocks }

    /// Get a reference to the inner [`NaiveChunk`] of this chunk.
    #[inline]
    #[must_use]
    pub const fn as_naive(&self) -> &NaiveChunk { &self.naive }

    /// Get the inner [`NaiveChunk`] of this chunk.
    #[inline]
    #[must_use]
    pub fn into_naive(self) -> NaiveChunk { self.naive }

    /// Get the height of this [`Chunk`].
    ///
    /// ## Note
    ///
    /// This is the height in world/coordinate space,
    /// and takes into account the chunk's vertical offset.
    #[inline]
    #[must_use]
    pub const fn height(&self) -> i32 { self.naive.height() }

    /// Get the height range of this [`Chunk`].
    ///
    /// ## Note
    ///
    /// This is the range in world/coordinate space and follows the chunk's
    /// vertical offset.
    #[inline]
    #[must_use]
    pub const fn height_range(&self) -> Range<i32> { self.naive.height_range() }

    /// Get the total height of this [`Chunk`], ignoring it's vertical offset.
    ///
    /// ## Note
    ///
    /// In other words, `y = 0` is always the bottom of the chunk.
    ///
    /// In most cases, you probably want [`Chunk::height`] instead.
    #[inline]
    #[must_use]
    pub const fn height_total(&self) -> usize { self.naive.height_total() }

    /// Get the height offset of this [`Chunk`].
    #[inline]
    #[must_use]
    pub const fn height_offset(&self) -> i32 { self.naive.height_offset() }

    /// Get a reference to the sections in this [`Chunk`].
    #[inline]
    #[must_use]
    pub const fn sections(&self) -> &[Section] { self.naive.sections() }

    /// Get a mutable reference to the sections in this [`Chunk`].
    #[inline]
    #[must_use]
    pub const fn sections_mut(&mut self) -> &mut [Section] { self.naive.sections_mut() }

    /// Get the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds, or if the [`Block`]
    /// does not exist in this [`Version`](froglight_common::version::Version).
    #[must_use]
    pub fn get_block<P: Into<BlockPos>>(&self, position: P) -> Option<Block> {
        self.naive.get_block_using::<P>(position, self.blocks())
    }

    /// Get the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds, or if the [`Block`]
    /// does not exist in this [`Version`](froglight_common::version::Version).
    #[must_use]
    pub fn get_block_pos<P: Into<ChunkBlockPos>>(&self, position: P) -> Option<Block> {
        self.naive.get_block_pos_using::<P>(position, self.blocks())
    }

    /// Set the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds, or if the [`Block`]
    /// does not exist in this [`Version`](froglight_common::version::Version).
    pub fn set_block<P: Into<BlockPos>>(&mut self, position: P, block: Block) -> Option<Block> {
        self.naive.set_block_using::<P>(position, block, self.blocks())
    }

    /// Set the [`Block`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds, or if the [`Block`]
    /// does not exist in this [`Version`](froglight_common::version::Version).
    pub fn set_block_pos<P: Into<ChunkBlockPos>>(
        &mut self,
        position: P,
        block: Block,
    ) -> Option<Block> {
        if self.blocks().version_ty() != block.version_ty() {
            return None;
        }

        self.naive.set_block_pos_using::<P>(position, block, self.blocks())
    }

    /// Get the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds, or if the [`Biome`]
    /// does not exist in this [`Version`](froglight_common::version::Version).
    #[must_use]
    pub fn get_biome<P: Into<BlockPos>>(&self, position: P) -> Option<Biome> {
        self.naive.get_biome_using::<P>(position, self.biomes())
    }

    /// Get the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds, or if the [`Biome`]
    /// does not exist in this [`Version`](froglight_common::version::Version).
    #[must_use]
    pub fn get_biome_pos<P: Into<ChunkBlockPos>>(&self, position: P) -> Option<Biome> {
        self.naive.get_biome_pos_using::<P>(position, self.biomes())
    }

    /// Set the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds, or if the [`Biome`]
    /// does not exist in this [`Version`](froglight_common::version::Version).
    pub fn set_biome<P: Into<BlockPos>>(&mut self, position: P, biome: Biome) -> Option<Biome> {
        self.naive.set_biome_using::<P>(position, biome, self.biomes())
    }

    /// Set the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds, or if the [`Biome`]
    /// does not exist in this [`Version`](froglight_common::version::Version).
    pub fn set_biome_pos<P: Into<ChunkBlockPos>>(
        &mut self,
        position: P,
        biome: Biome,
    ) -> Option<Biome> {
        if self.biomes().version_ty() != biome.version_ty() {
            return None;
        }

        self.naive.set_biome_pos_using::<P>(position, biome, self.biomes)
    }

    /// Convert this [`Chunk`] into another version.
    ///
    /// Attempts to use [`Stone`](froglight_block::prelude::block::Stone) for
    /// blocks and [`Plains`](froglight_biome::prelude::biome::Plains) for
    /// biomes that cannot be converted.
    pub fn convert_into<V: BiomeVersion + BlockVersion>(&mut self) {
        self.convert_biomes::<V>();
        self.convert_blocks::<V>();
    }

    #[inline(always)]
    #[expect(clippy::inline_always, reason = "Optimizations")]
    fn convert_biomes<V: BiomeVersion>(&mut self) {
        // Skip if the chunk is already in the correct version.
        if self.biomes().version_ty() == TypeId::of::<V>() {
            return;
        }

        let old = self.biomes;
        let new = V::biomes();

        // Fallback to `minecraft:plains`, which is usually `0`.
        let fallback = new
            .get_biome_by_identifier(&Identifier::new_static("minecraft:plains"))
            .map_or(0, |biome| biome.global_id().into_inner());

        let mut cache = SmallVec::<[(u32, u32); 15]>::new();
        let mut convert_id = |id: u32| -> u32 {
            if let Some((_, cached)) = cache.iter().find(|(cached, _)| id == *cached) {
                *cached
            } else if let Some(biome) = old.get_biome_by_id(GlobalBiomeId::new(id))
                && let Some(biome) = biome.using_version::<V>()
            {
                cache.push((id, biome.global_id().into_inner()));
                biome.global_id().into_inner()
            } else {
                fallback
            }
        };

        for section in self.sections_mut() {
            let biome = section.biome_data_mut();

            // SAFETY: We guarantee that all biome ids via `convert_id` are valid.
            unsafe {
                match biome.palette_mut() {
                    SectionPalette::Single(biome_id) => {
                        *biome_id = convert_id(*biome_id);
                    }
                    SectionPalette::Vector(biome_ids) => {
                        for biome_id in biome_ids {
                            *biome_id = convert_id(*biome_id);
                        }
                    }
                    SectionPalette::Global => {
                        // Iterate over each biome index and convert it.
                        // SAFETY: `index` is always within bounds `0..BiomeSection::VOLUME`.
                        for index in (0..BiomeSection::VOLUME).map(usize::from) {
                            let biome_id = biome.get_index(index).unwrap_unchecked();
                            biome.set_index(index, convert_id(biome_id));
                        }
                    }
                }
            }
        }

        // Use the new version's biome storage.
        self.biomes = new;
    }

    #[inline(always)]
    #[expect(clippy::inline_always, reason = "Optimizations")]
    fn convert_blocks<V: BlockVersion>(&mut self) {
        // Skip if the chunk is already in the correct version.
        if self.blocks().version_ty() == TypeId::of::<V>() {
            return;
        }

        let old = self.blocks;
        let new = V::blocks();

        // Fallback to `minecraft:stone`, which is usually `1`.
        let fallback = new
            .get_block_by_identifier(&Identifier::new_static("minecraft:stone"))
            .map_or(1, |biome| biome.global_id().into_inner());

        let mut cache = SmallVec::<[(u32, u32); 15]>::new();
        let mut convert_id = |id: u32| -> u32 {
            if let Some((_, cached)) = cache.iter().find(|(cached, _)| id == *cached) {
                *cached
            } else if let Some(block) = old.get_block_by_state(GlobalStateId::new(id))
                && let Some(blocks) = block.using_version::<V>()
            {
                cache.push((id, blocks.global_id().into_inner()));
                blocks.global_id().into_inner()
            } else {
                fallback
            }
        };

        for section in self.sections_mut() {
            let biome = section.biome_data_mut();

            // SAFETY: We guarantee that all blockstate ids via `convert_id` are valid.
            unsafe {
                match biome.palette_mut() {
                    SectionPalette::Single(biome_id) => {
                        *biome_id = convert_id(*biome_id);
                    }
                    SectionPalette::Vector(biome_ids) => {
                        for biome_id in biome_ids {
                            *biome_id = convert_id(*biome_id);
                        }
                    }
                    SectionPalette::Global => {
                        // Iterate over each biome index and convert it.
                        // SAFETY: `index` is always within bounds `0..BiomeSection::VOLUME`.
                        for index in (0..BiomeSection::VOLUME).map(usize::from) {
                            let biome_id = biome.get_index(index).unwrap_unchecked();
                            biome.set_index(index, convert_id(biome_id));
                        }
                    }
                }
            }
        }

        // Use the new version's block storage.
        self.blocks = new;
    }

    #[inline(always)]
    #[expect(clippy::inline_always, reason = "Optimizations")]
    fn eq_biomes(&self, other: &Self) -> bool {
        if self.sections().len() != other.sections().len() {
            return false;
        }

        if self.biomes().version_ty() == other.biomes().version_ty() {
            // If the versions are the same we can just compare the data directly.
            self.sections()
                .iter()
                .zip(other.sections().iter())
                .all(|(a, b)| a.biome_data() == b.biome_data())
        } else {
            let self_biomes = self.biomes();
            let other_biomes = other.biomes();

            // Create a closure to compare the biome ids.
            let compare_ab = |a: u32, b: u32| -> bool {
                other_biomes
                    .get_biome_by_id(GlobalBiomeId::new(b))
                    .and_then(|b| b.using_version_storage(self_biomes))
                    .is_some_and(|b| b.global_id() == a)
            };

            for (a, b) in self.sections().iter().zip(other.sections().iter()) {
                let a = a.biome_data();
                let b = b.biome_data();

                // Compare the palettes
                match (a.palette(), b.palette()) {
                    (SectionPalette::Single(a), SectionPalette::Single(b)) => {
                        if !compare_ab(*a, *b) {
                            return false;
                        }
                    }
                    (SectionPalette::Vector(a), SectionPalette::Vector(b)) => {
                        for (a, b) in a.iter().zip(b.iter()) {
                            if !compare_ab(*a, *b) {
                                return false;
                            }
                        }
                    }
                    (SectionPalette::Global, SectionPalette::Global) => {}
                    _ => return false,
                }

                // Compare the data
                if a.data() != b.data() {
                    return false;
                }
            }

            true
        }
    }

    #[inline(always)]
    #[expect(clippy::inline_always, reason = "Optimizations")]
    fn eq_blocks(&self, other: &Self) -> bool {
        if self.sections().len() != other.sections().len() {
            return false;
        }

        if self.blocks().version_ty() == other.blocks().version_ty() {
            // If the versions are the same we can just compare the data directly.
            self.sections()
                .iter()
                .zip(other.sections().iter())
                .all(|(a, b)| a.block_data() == b.block_data())
        } else {
            let self_blocks = self.blocks();
            let other_blocks = other.blocks();

            // Create a closure to compare the block ids.
            let compare_ab = |a: u32, b: u32| -> bool {
                other_blocks
                    .get_block_by_state(GlobalStateId::new(b))
                    .and_then(|b| b.using_version_storage(self_blocks))
                    .is_some_and(|b| b.global_id() == a)
            };

            for (a, b) in self.sections().iter().zip(other.sections().iter()) {
                // Compare the block and fluid counts
                if a.block_count() != b.block_count() || a.fluid_count() != b.fluid_count() {
                    return false;
                }

                let a = a.block_data();
                let b = b.block_data();

                // Compare the palettes
                match (a.palette(), b.palette()) {
                    (SectionPalette::Single(a), SectionPalette::Single(b)) => {
                        if !compare_ab(*a, *b) {
                            return false;
                        }
                    }
                    (SectionPalette::Vector(a), SectionPalette::Vector(b)) => {
                        for (a, b) in a.iter().zip(b.iter()) {
                            if !compare_ab(*a, *b) {
                                return false;
                            }
                        }
                    }
                    (SectionPalette::Global, SectionPalette::Global) => {}
                    _ => return false,
                }

                // Compare the data
                if a.data() != b.data() {
                    return false;
                }
            }

            true
        }
    }
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Chunk").finish_non_exhaustive()
    }
}

impl Eq for Chunk {}
impl PartialEq for Chunk {
    fn eq(&self, other: &Self) -> bool { self.eq_biomes(other) && self.eq_blocks(other) }
}
