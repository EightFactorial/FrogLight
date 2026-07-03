//! TODO

use core::any::TypeId;

use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use indexmap::IndexMap;

use crate::{
    biome::{Biome, BiomeMetadata},
    state::GlobalBiomeId,
    version::BiomeVersion,
};

/// A container for biome data storage.
#[derive(Debug, Clone)]
pub struct BiomeStorage {
    version: TypeId,
    metadata: IndexMap<Identifier<'static>, &'static BiomeMetadata, RandomState>,
}

impl BiomeStorage {
    /// Build a new [`BiomeStorage`] for the given [`BiomeVersion`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that all provided biome metadata has the correct
    /// global ids for this collection.
    #[must_use]
    pub unsafe fn build<V: BiomeVersion>(metadata: &[&'static BiomeMetadata]) -> Self {
        let mut identifiers =
            IndexMap::with_capacity_and_hasher(metadata.len(), RandomState::default());

        for &meta in metadata {
            identifiers.entry(meta.identifier().reborrow()).insert_entry(meta);
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
        self.metadata.get_index(id.into_inner() as usize).map(|(_, meta)| Biome::new_from(meta))
    }

    /// Get the [`Biome`] for a given [`Identifier`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry.
    #[must_use]
    pub fn get_biome_by_identifier(&self, identifier: &Identifier<'_>) -> Option<Biome> {
        self.metadata.get(identifier).map(|meta| Biome::new_from(meta))
    }

    /// Get the [`TypeId`] of the [`Version`] this storage is for.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version }

    /// Get the [`IndexMap`] metadata of this [`BiomeStorage`].
    #[inline]
    #[must_use]
    pub const fn metadata(
        &self,
    ) -> &IndexMap<Identifier<'static>, &'static BiomeMetadata, RandomState> {
        &self.metadata
    }

    /// Get the mutable [`IndexMap`] metadata of this [`BiomeStorage`].
    #[inline]
    #[must_use]
    pub fn metadata_mut(
        &mut self,
    ) -> &mut IndexMap<Identifier<'static>, &'static BiomeMetadata, RandomState> {
        &mut self.metadata
    }
}
