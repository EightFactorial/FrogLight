//! TODO

use alloc::vec::Vec;
use core::any::TypeId;

use foldhash::fast::RandomState;
use froglight_common::identifier::Identifier;
use indexmap::IndexMap;

use crate::{
    item::{Item, ItemMetadata},
    state::GlobalItemId,
    version::ItemVersion,
};

/// A container for item data storage.
#[derive(Debug, Clone)]
pub struct ItemStorage {
    version: TypeId,
    metadata: IndexMap<Identifier<'static>, &'static ItemMetadata, RandomState>,
}

impl ItemStorage {
    /// Get the default [`Item`] for a given [`GlobalItemId`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry and world.
    #[must_use]
    pub fn get_item_by_id(&self, id: GlobalItemId) -> Option<Item> {
        self.metadata.get_index(id.into_inner() as usize).map(|(_, meta)| Item::new_from(meta))
    }

    /// Get the default [`Item`] for a given [`Identifier`].
    ///
    /// # Note
    ///
    /// This is typically used by the registry.
    #[must_use]
    pub fn get_item_by_identifier(&self, identifier: &Identifier<'_>) -> Option<Item> {
        self.metadata.get(identifier).map(|meta| Item::new_from(meta))
    }

    /// Get the [`TypeId`] of the [`Version`] this storage is for.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version }

    /// Get the [`IndexMap`] metadata of this [`ItemStorage`].
    #[inline]
    #[must_use]
    pub const fn metadata(
        &self,
    ) -> &IndexMap<Identifier<'static>, &'static ItemMetadata, RandomState> {
        &self.metadata
    }

    /// Build a new [`ItemStorage`] for the given [`ItemVersion`].
    #[must_use]
    pub fn build<V: ItemVersion>(metadata: Vec<&'static ItemMetadata>) -> Self {
        let mut identifiers =
            IndexMap::with_capacity_and_hasher(metadata.len(), RandomState::default());

        for meta in metadata {
            identifiers.entry(meta.identifier().reborrow()).insert_entry(meta);
        }

        Self { version: TypeId::of::<V>(), metadata: identifiers }
    }
}
