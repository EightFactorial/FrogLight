//! Additional methods that require the [`froglight_biome`] crate.

use core::any::TypeId;

#[cfg(feature = "std")]
use froglight_biome::biome::BiomeType;
use froglight_biome::{biome::GlobalId, prelude::*, storage::BiomeStorage};

use crate::{
    chunk::{NaiveChunk, section::SectionPalette},
    component::ChunkBlockPos,
    prelude::*,
};

impl NaiveChunk {
    /// Get the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized by the
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    #[cfg(feature = "std")]
    pub fn get_biome<V: BiomeVersion, P: Into<BlockPos>>(&self, position: P) -> Option<Biome> {
        self.get_biome_using::<P>(position, &V::biomes().load())
    }

    /// Get the [`Biome`] at the given position within the chunk,
    /// resolving it using the provided [`BiomeStorage`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized by the [`BiomeStorage`].
    #[must_use]
    pub fn get_biome_using<P: Into<BlockPos>>(
        &self,
        position: P,
        storage: &BiomeStorage,
    ) -> Option<Biome> {
        ChunkBlockPos::try_from_blockpos(position.into(), self.height_offset())
            .and_then(|pos| self.get_biome_pos_using::<ChunkBlockPos>(pos, storage))
    }

    /// Get the [`Biome`] at the given position within the chunk.
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized by the
    /// [`Version`](froglight_common::version::Version).
    #[must_use]
    #[cfg(feature = "std")]
    pub fn get_biome_pos<V: BiomeVersion, P: Into<ChunkBlockPos>>(
        &self,
        position: P,
    ) -> Option<Biome> {
        self.get_biome_pos_using::<P>(position, &V::biomes().load())
    }

    /// Get the [`Biome`] at the given position within the chunk,
    /// resolving it using the provided [`BiomeStorage`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized by the [`BiomeStorage`].
    #[must_use]
    pub fn get_biome_pos_using<P: Into<ChunkBlockPos>>(
        &self,
        position: P,
        storage: &BiomeStorage,
    ) -> Option<Biome> {
        self.get_raw_biome_pos::<P>(position).and_then(|id| storage.get_biome(GlobalId::new(id)))
    }

    /// Set the [`Biome`] at the given position within the chunk,
    /// returning the previous [`Biome`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized by the
    /// [`Version`](froglight_common::version::Version).
    #[cfg(feature = "std")]
    pub fn set_biome<V: BiomeVersion, P: Into<BlockPos>>(
        &mut self,
        position: P,
        biome: Biome,
    ) -> Option<Biome> {
        self.set_biome_using::<P>(position, biome, &V::biomes().load())
    }

    /// Get the [`Biome`] at the given position within the chunk and return the
    /// previous one, resolving it using the provided [`BiomeStorage`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized by the [`BiomeStorage`].
    pub fn set_biome_using<P: Into<BlockPos>>(
        &mut self,
        position: P,
        biome: Biome,
        storage: &BiomeStorage,
    ) -> Option<Biome> {
        ChunkBlockPos::try_from_blockpos(position.into(), self.height_offset())
            .and_then(|pos| self.set_biome_pos_using::<ChunkBlockPos>(pos, biome, storage))
    }

    /// Set the [`Biome`] at the given position within the chunk,
    /// returning the previous [`Biome`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized by the
    /// [`Version`](froglight_common::version::Version).
    #[cfg(feature = "std")]
    pub fn set_biome_pos<V: BiomeVersion, P: Into<ChunkBlockPos>>(
        &mut self,
        position: P,
        biome: Biome,
    ) -> Option<Biome> {
        self.set_biome_pos_using::<P>(position, biome, &V::biomes().load())
    }

    /// Get the [`Biome`] at the given position within the chunk and return the
    /// previous one, resolving it using the provided [`BiomeStorage`].
    ///
    /// Returns `None` if the position is out of bounds,
    /// or if the biome is not recognized by the [`BiomeStorage`].
    pub fn set_biome_pos_using<P: Into<ChunkBlockPos>>(
        &mut self,
        position: P,
        biome: Biome,
        storage: &BiomeStorage,
    ) -> Option<Biome> {
        self.set_raw_biome_pos::<P>(position, biome.global_id().into_inner())
            .and_then(|id| storage.get_biome(GlobalId::new(id)))
    }

    /// Returns `true` if the chunk contains at least one biome of the same
    /// type.
    ///
    /// Resolves biome types using the provided [`BiomeStorage`].
    #[must_use]
    pub fn contains_biome(&self, biome: Biome) -> bool {
        self.contains_raw_biome(biome.global_id().into_inner())
    }

    /// Returns `true` if the chunk contains at least one biome of the same
    /// type.
    #[must_use]
    pub fn contains_biome_type<B: BiomeType<V>, V: BiomeVersion>(&self) -> bool {
        self.contains_raw_biome(B::METADATA.global_id().into_inner())
    }

    /// Returns `true` if the chunk contains at least one biome of the same
    /// type.
    #[must_use]
    pub fn contains_biome_type_using(&self, biome_type: TypeId, storage: &BiomeStorage) -> bool {
        let Some(meta) = storage.to_ref().iter().find(|biome| biome.biome_ty() == biome_type)
        else {
            return false;
        };

        let biome_id = meta.global_id().into_inner();
        self.storage.as_slice().iter().any(|section| match section.biome_data().palette() {
            SectionPalette::Single(id) => *id == biome_id,
            SectionPalette::Vector(vec) => vec.iter().any(|palette_id| {
                if *palette_id == biome_id {
                    // Cannot return `true` directly as the palette may contain unused values.
                    section.iter_raw_biomes().any(|id| id == biome_id)
                } else {
                    false
                }
            }),
            SectionPalette::Global => section.iter_raw_biomes().any(|id| id == biome_id),
        })
    }
}
