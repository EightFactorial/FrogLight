//! TODO

use alloc::vec::Vec;
use core::any::TypeId;

use foldhash::fast::RandomState;
use froglight_common::identifier::Identifier;
use froglight_nbt::types::indexed::alloc::IndexedNbtCow;
use indexmap::IndexMap;

use crate::{
    registry::{NbtRef, TagRef},
    state::GlobalRegistryId,
    version::RegistryVersion,
};

/// A container for registry data storage.
#[derive(Debug, Clone)]
pub struct RegistryStorage {
    version: TypeId,
    tag_data: IndexMap<
        Identifier<'static>,
        IndexMap<Identifier<'static>, Vec<u32>, RandomState>,
        RandomState,
    >,
    nbt_data: IndexMap<
        Identifier<'static>,
        IndexMap<Identifier<'static>, IndexedNbtCow<'static>, RandomState>,
        RandomState,
    >,
}

impl RegistryStorage {
    /// Get the [`NbtRef`] for a given [`GlobalRegistryId`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry and world.
    #[must_use]
    pub fn get_nbt_by_id(&self, id: GlobalRegistryId) -> Option<NbtRef<'_>> {
        self.nbt_data
            .get_index(id.into_inner() as usize)
            .map(|(identifier, values)| NbtRef::new(identifier.reborrow(), values))
    }

    /// Get the [`NbtRef`] for a given [`Identifier`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry.
    #[must_use]
    pub fn get_nbt_by_identifier<'a>(&'a self, identifier: &str) -> Option<NbtRef<'a>> {
        self.nbt_data
            .get_key_value(identifier)
            .map(|(identifier, values)| NbtRef::new(identifier.reborrow(), values))
    }

    /// Get the [`TagRef`] for a given [`GlobalRegistryId`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry and world.
    #[must_use]
    pub fn get_tag_by_id(&self, id: GlobalRegistryId) -> Option<TagRef<'_>> {
        self.tag_data
            .get_index(id.into_inner() as usize)
            .map(|(identifier, values)| TagRef::new(identifier.reborrow(), values))
    }

    /// Get the [`TagReg`] for a given [`Identifier`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry.
    #[must_use]
    pub fn get_registry_by_identifier<'a>(&'a self, identifier: &str) -> Option<TagRef<'a>> {
        self.tag_data
            .get_key_value(identifier)
            .map(|(identifier, values)| TagRef::new(identifier.reborrow(), values))
    }

    /// Get the [`TypeId`] of the [`Version`] this storage is for.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version }

    /// Get the [`IndexMap`] of tag data in this [`RegistryStorage`].
    #[inline]
    #[must_use]
    pub const fn tags(
        &self,
    ) -> &IndexMap<
        Identifier<'static>,
        IndexMap<Identifier<'static>, Vec<u32>, RandomState>,
        RandomState,
    > {
        &self.tag_data
    }

    /// Get the mutable [`IndexMap`] of tag data in this [`RegistryStorage`].
    #[inline]
    #[must_use]
    pub const fn tags_mut(
        &mut self,
    ) -> &mut IndexMap<
        Identifier<'static>,
        IndexMap<Identifier<'static>, Vec<u32>, RandomState>,
        RandomState,
    > {
        &mut self.tag_data
    }

    /// Get the [`IndexMap`] of NBT data in this [`RegistryStorage`].
    #[inline]
    #[must_use]
    pub const fn nbt(
        &self,
    ) -> &IndexMap<
        Identifier<'static>,
        IndexMap<Identifier<'static>, IndexedNbtCow<'static>, RandomState>,
        RandomState,
    > {
        &self.nbt_data
    }

    /// Get the mutable [`IndexMap`] of NBT data in this [`RegistryStorage`].
    #[inline]
    #[must_use]
    pub const fn nbt_mut(
        &mut self,
    ) -> &mut IndexMap<
        Identifier<'static>,
        IndexMap<Identifier<'static>, IndexedNbtCow<'static>, RandomState>,
        RandomState,
    > {
        &mut self.nbt_data
    }

    /// Build a new [`RegistryStorage`] for the given [`RegistryVersion`].
    #[must_use]
    #[expect(clippy::type_complexity, reason = "Nested `Vec`s")]
    pub fn build<V: RegistryVersion>(
        tags: Vec<(Identifier<'static>, Vec<(Identifier<'static>, Vec<u32>)>)>,
        nbt: Vec<(Identifier<'static>, Vec<(Identifier<'static>, IndexedNbtCow<'static>)>)>,
    ) -> Self {
        let mut tag_data = IndexMap::with_capacity_and_hasher(tags.len(), RandomState::default());
        let mut nbt_data = IndexMap::with_capacity_and_hasher(nbt.len(), RandomState::default());

        for (key, values) in tags {
            tag_data.entry(key).insert_entry(values.into_iter().collect());
        }
        for (key, values) in nbt {
            nbt_data.entry(key).insert_entry(values.into_iter().collect());
        }

        Self { version: TypeId::of::<V>(), tag_data, nbt_data }
    }
}
