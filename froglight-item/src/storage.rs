//! TODO

use core::any::TypeId;

use froglight_common::{crates::indexmap::map::Entry, prelude::*, types::IndexMap};

use crate::{
    item::{Item, ItemMetadata},
    state::GlobalItemId,
    version::ItemVersion,
};

/// A container for item data storage.
#[derive(Debug, Clone)]
pub struct ItemStorage {
    version: TypeId,
    metadata: IndexMap<&'static Ident, &'static ItemMetadata>,
}

impl ItemStorage {
    /// Build a new [`ItemStorage`] for the given [`ItemVersion`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that all provided [`ItemMetadata`] is valid for
    /// this [`Version`], and has a matching entry per-[`GlobalItemId`].
    ///
    /// # Panics
    ///
    /// Panics in `debug` builds if the provided [`ItemMetadata`] is invalid.
    #[must_use]
    pub unsafe fn build<V: ItemVersion>(metadata: &[&'static ItemMetadata]) -> Self {
        let mut identifiers = IndexMap::with_capacity_and_hasher(metadata.len(), <_>::default());

        for (_index, meta) in metadata.iter().enumerate() {
            #[cfg(debug_assertions)]
            #[expect(clippy::used_underscore_binding, reason = "Debug assertions")]
            Self::assert_item::<V>(_index, meta);

            match identifiers.entry(meta.identifier()) {
                Entry::Vacant(entry) => _ = entry.insert(*meta),
                Entry::Occupied(..) => {
                    core::hint::cold_path();

                    #[cfg(debug_assertions)]
                    panic!("ItemMetadata has duplicate identifier: {:?}", meta.identifier());
                }
            }
        }

        Self { version: TypeId::of::<V>(), metadata: identifiers }
    }

    #[cfg(debug_assertions)]
    fn assert_item<V: ItemVersion>(index: usize, meta: &ItemMetadata) {
        debug_assert_eq!(
            index,
            meta.global_id().into_usize(),
            "GlobalItemId `{index}` does not match the expected value for {:?}: `{}`",
            meta.identifier(),
            meta.global_id().into_inner(),
        );

        debug_assert!(
            meta.is_version::<V>(),
            "ItemMetadata Version mismatch for {:?}: expected {:?}",
            meta.identifier(),
            core::any::type_name::<V>()
        );
    }

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
    pub fn get_item_by_identifier(&self, identifier: &Ident) -> Option<Item> {
        self.metadata.get(identifier).map(|meta| Item::new_from(meta))
    }

    /// Get the [`TypeId`] of the [`Version`] this storage is for.
    #[inline]
    #[must_use]
    pub const fn version_ty(&self) -> TypeId { self.version }

    /// Get the [`IndexMap`] metadata of this [`ItemStorage`].
    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &IndexMap<&'static Ident, &'static ItemMetadata> {
        &self.metadata
    }
}
