//! Additional methods that require the [`froglight_biome`] crate.

use core::any::TypeId;

use froglight_biome::{prelude::*, storage::BiomeStorage};

use crate::{component::ChunkBlockPos, prelude::*, section::SectionPalette};

impl NaiveChunk {
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
        self.get_raw_biome_pos::<P>(position)
            .and_then(|id| storage.get_biome_by_id(GlobalBiomeId::new(id)))
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
        let biome_id = biome.using_version_storage(storage)?.global_id().into_inner();
        self.set_raw_biome_pos::<P>(position, biome_id)
            .and_then(|id| storage.get_biome_by_id(GlobalBiomeId::new(id)))
    }

    /// Returns `true` if the chunk contains at least one biome of the same
    /// type.
    #[must_use]
    pub fn contains_biome(&self, biome: Biome) -> bool {
        self.contains_raw_biome(biome.global_id().into_inner())
    }

    /// Returns `true` if the chunk contains at least one biome of the same
    /// type.
    #[must_use]
    pub fn contains_biome_type(&self, biome_type: TypeId, storage: &BiomeStorage) -> bool {
        let Some(biome_id) = storage.metadata().iter().find_map(|(_, meta)| {
            if meta.biome_ty() == biome_type { Some(meta.global_id().into_inner()) } else { None }
        }) else {
            return false;
        };

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
