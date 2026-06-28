//! TODO

use alloc::vec::Vec;
use core::any::TypeId;

use foldhash::fast::RandomState;
use froglight_common::identifier::Identifier;
use indexmap::IndexMap;

use crate::{registry::RegistryRef, state::GlobalRegistryId, version::RegistryVersion};

/// A container for registry data storage.
#[derive(Debug, Clone)]
pub struct RegistryStorage {
    version: TypeId,
    metadata: IndexMap<
        Identifier<'static>,
        IndexMap<Identifier<'static>, Vec<u32>, RandomState>,
        RandomState,
    >,
}

impl RegistryStorage {
    /// Get the [`Registry`] for a given [`GlobalRegistryId`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry and world.
    #[must_use]
    pub fn get_registry_by_id(&self, id: GlobalRegistryId) -> Option<RegistryRef<'_>> {
        self.metadata
            .get_index(id.into_inner() as usize)
            .map(|(identifier, values)| RegistryRef::new(identifier.reborrow(), values))
    }

    /// Get the [`Registry`] for a given [`Identifier`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry.
    #[must_use]
    pub fn get_registry_by_identifier<'a>(&'a self, identifier: &str) -> Option<RegistryRef<'a>> {
        self.metadata
            .get_key_value(identifier)
            .map(|(identifier, values)| RegistryRef::new(identifier.reborrow(), values))
    }

    /// Get the [`TypeId`] of the [`Version`] this storage is for.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version }

    /// Get the [`IndexMap`] metadata of this [`RegistryStorage`].
    #[inline]
    #[must_use]
    pub const fn metadata(
        &self,
    ) -> &IndexMap<
        Identifier<'static>,
        IndexMap<Identifier<'static>, Vec<u32>, RandomState>,
        RandomState,
    > {
        &self.metadata
    }

    /// Build a new [`RegistryStorage`] for the given [`RegistryVersion`].
    #[must_use]
    pub fn build<V: RegistryVersion>(
        metadata: Vec<(Identifier<'static>, Vec<(Identifier<'static>, Vec<u32>)>)>,
    ) -> Self {
        let mut identifiers =
            IndexMap::with_capacity_and_hasher(metadata.len(), RandomState::default());

        for (key, values) in metadata {
            identifiers.entry(key).insert_entry(values.into_iter().collect());
        }

        Self { version: TypeId::of::<V>(), metadata: identifiers }
    }
}
