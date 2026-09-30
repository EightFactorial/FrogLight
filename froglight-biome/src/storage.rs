//! TODO

use core::any::TypeId;

use froglight_common::{crates::indexmap::map::Entry, prelude::*, types::IndexMap};

use crate::{
    biome::{Biome, BiomeMetadata},
    state::GlobalBiomeId,
    version::BiomeVersion,
};

/// A container for biome data storage.
#[derive(Debug, Clone)]
pub struct BiomeStorage {
    version: TypeId,
    metadata: IndexMap<&'static Ident, &'static BiomeMetadata>,
}

impl BiomeStorage {
    /// Build a new [`BiomeStorage`] for the given [`BiomeVersion`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that all provided [`BiomeMetadata`] is valid for
    /// this [`Version`], and has a matching entry per-[`GlobalBiomeId`].
    ///
    /// # Panics
    ///
    /// Panics in `debug` builds if the provided [`BiomeMetadata`] is invalid.
    #[must_use]
    pub unsafe fn build<V: BiomeVersion>(metadata: &[&'static BiomeMetadata]) -> Self {
        let mut identifiers = IndexMap::with_capacity_and_hasher(metadata.len(), <_>::default());

        for (_index, meta) in metadata.iter().enumerate() {
            #[cfg(debug_assertions)]
            #[expect(clippy::used_underscore_binding, reason = "Debug assertions")]
            Self::assert_biome::<V>(_index, meta);

            match identifiers.entry(meta.identifier()) {
                Entry::Vacant(entry) => _ = entry.insert(*meta),
                Entry::Occupied(..) => {
                    core::hint::cold_path();

                    #[cfg(debug_assertions)]
                    panic!("BiomeMetadata has duplicate identifier: {:?}", meta.identifier());
                }
            }
        }

        Self { version: TypeId::of::<V>(), metadata: identifiers }
    }

    #[cfg(debug_assertions)]
    fn assert_biome<V: BiomeVersion>(index: usize, meta: &BiomeMetadata) {
        debug_assert_eq!(
            index,
            meta.global_id().into_usize(),
            "GlobalBiomeId `{index}` does not match the expected value for {:?}: `{}`",
            meta.identifier(),
            meta.global_id().into_inner(),
        );

        debug_assert!(
            meta.is_version::<V>(),
            "BiomeMetadata Version mismatch for {:?}: expected {:?}",
            meta.identifier(),
            core::any::type_name::<V>()
        );
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
    pub const fn metadata(&self) -> &IndexMap<&'static Ident, &'static BiomeMetadata> {
        &self.metadata
    }

    /// Get the mutable [`IndexMap`] metadata of this [`BiomeStorage`].
    #[inline]
    #[must_use]
    pub fn metadata_mut(&mut self) -> &mut IndexMap<&'static Ident, &'static BiomeMetadata> {
        &mut self.metadata
    }
}
