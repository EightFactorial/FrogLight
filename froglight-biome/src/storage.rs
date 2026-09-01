//! TODO

use core::any::TypeId;

use foldhash::fast::RandomState;
use froglight_common::prelude::*;
use indexmap::{IndexMap, map::Entry};

use crate::{
    biome::{Biome, BiomeMetadata},
    state::GlobalBiomeId,
    version::BiomeVersion,
};

/// A container for biome data storage.
#[derive(Debug, Clone)]
pub struct BiomeStorage {
    version: TypeId,
    metadata: IndexMap<&'static Ident, &'static BiomeMetadata, RandomState>,
}

impl BiomeStorage {
    /// Build a new [`BiomeStorage`] for the given [`BiomeVersion`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that all provided biome metadata has the correct
    /// global ids for this collection.
    ///
    /// # Panics
    ///
    /// Panics if there are duplicate biome identifiers in the provided
    /// metadata, or if any of the metadata belongs to a different
    /// [`BiomeVersion`].
    #[must_use]
    pub unsafe fn build<V: BiomeVersion>(metadata: &[&'static BiomeMetadata]) -> Self {
        let mut identifiers =
            IndexMap::with_capacity_and_hasher(metadata.len(), RandomState::default());

        for meta in metadata {
            if !meta.is_version::<V>() {
                core::hint::cold_path();
                panic!("BiomeMetadata version mismatch: expected {}", core::any::type_name::<V>());
            }

            match identifiers.entry(meta.identifier()) {
                Entry::Vacant(entry) => _ = entry.insert(*meta),
                Entry::Occupied(..) => {
                    core::hint::cold_path();
                    panic!("BiomeMetadata has duplicate identifier: {:?}", meta.identifier());
                }
            }
        }

        Self { version: TypeId::of::<V>(), metadata: identifiers }
    }

    /// Get the [`Biome`] for a given [`GlobalStateId`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry and world.
    #[must_use]
    pub fn get_biome_by_id(&self, id: GlobalBiomeId) -> Option<Biome> {
        if let Some((_, meta)) = self.metadata.get_index(id.into_inner() as usize) {
            Some(Biome::new_from(meta))
        } else {
            core::hint::cold_path();
            None
        }
    }

    /// Get the [`Biome`] for a given [`Identifier`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry.
    #[must_use]
    pub fn get_biome_by_identifier(&self, identifier: &Ident) -> Option<Biome> {
        if let Some(meta) = self.metadata.get(identifier) {
            Some(Biome::new_from(meta))
        } else {
            core::hint::cold_path();
            None
        }
    }

    /// Get the [`TypeId`] of the [`Version`] this storage is for.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version }

    /// Get the [`IndexMap`] metadata of this [`BiomeStorage`].
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &IndexMap<&'static Ident, &'static BiomeMetadata, RandomState> {
        &self.metadata
    }

    /// Get the mutable [`IndexMap`] metadata of this [`BiomeStorage`].
    #[inline]
    #[must_use]
    pub fn metadata_mut(
        &mut self,
    ) -> &mut IndexMap<&'static Ident, &'static BiomeMetadata, RandomState> {
        &mut self.metadata
    }
}
