//! [`ItemStorage`] and [`AppItemStorage`]

use alloc::sync::Arc;
use core::any::TypeId;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use froglight_common::{vanilla::Vanilla, version::Version};
use froglight_nbt::nbt::UnnamedNbt;
use froglight_utils::storage::prelude::*;

use crate::{
    item::{ItemType, ItemTypeExt, UntypedItem},
    resolve::ItemResolver,
};

/// A dynamic storage for item types.
///
/// Allows for the registration and retrieval of item types at runtime.
#[repr(transparent)]
#[derive(Clone, AppStorage)]
#[storage(index(ident = "GlobalItemId", inner = "u32"), bevy = "bevy", reflect = "reflect")]
pub struct ItemStorage<V: Version>(IndexedLocalStorage<dyn ItemType<V>, GlobalItemId>);

impl<V: Version> AppItemStorage<V> {
    /// Create a new [`AppItemStorage`] with the [`Vanilla`] types registered.
    #[inline]
    #[must_use]
    pub fn new() -> Self
    where Vanilla: ItemResolver<V> {
        Self::from_storage(ItemStorage::new())
    }
}

impl<V: Version> ItemStorage<V> {
    /// Create a new [`ItemStorage`] with the [`Vanilla`] types
    /// registered.
    #[must_use]
    pub fn new() -> Self
    where Vanilla: ItemResolver<V> {
        let mut storage = Self::new_empty();
        <Vanilla as ItemResolver<V>>::register(&mut storage);
        storage
    }

    /// Create a new [`ItemStorage`] with no registered item types.
    #[must_use]
    pub const fn new_empty() -> Self { Self(IndexedLocalStorage::new()) }

    /// Get the [`ItemType`] for the given [`GlobalItemId`].
    ///
    /// Handy for storing many item types and bulk operations.
    ///
    /// Returns `None` if no item with the given id was registered.
    #[must_use]
    pub fn get_trait(&self, item_id: GlobalItemId) -> Option<&'static dyn ItemType<V>> {
        self.0.get_index(item_id).map(|val| val.inner())
    }

    /// Get the [`UntypedItem`] for the given [`GlobalItemId`].
    ///
    /// If no data is provided, the item's default NBT data will be used.
    ///
    /// Returns `None` if no item with the given id was registered.
    #[must_use]
    pub fn get_untyped(
        &self,
        item_id: GlobalItemId,
        data: Option<UnnamedNbt>,
    ) -> Option<UntypedItem<V>> {
        self.0
            .get_index(item_id)
            .map(|val| UntypedItem::<V>::new(data.unwrap_or_else(|| val.default_nbt()), val))
    }

    /// Get a typed item for the given [`GlobalItemId`].
    ///
    /// Returns `None` if no item with the given id was registered,
    /// `Ok` if the item was registered and resolved successfully,
    /// and `Err` if the item was registered but could not be resolved.
    #[inline]
    #[must_use]
    pub fn get_typed<R: ItemResolver<V>>(
        &self,
        item_id: GlobalItemId,
        data: Option<UnnamedNbt>,
    ) -> Option<Result<R::ItemEnum, UntypedItem<V>>> {
        self.get_untyped(item_id, data).map(R::resolve)
    }

    /// Get the [`GlobalItemId`] for the given [`ItemType`].
    ///
    /// Returns `None` if the item was not registered.
    #[inline]
    #[must_use]
    pub fn get_global_id<I: ItemType<V>>(&self) -> Option<GlobalItemId> {
        self.get_global_type_id(&TypeId::of::<I>())
    }

    /// Get the [`GlobalItemId`] for the given [`UntypedItem`].
    ///
    /// Returns `None` if the item was not registered.
    #[inline]
    #[must_use]
    pub fn get_global_untyped_id(&self, item: &UntypedItem<V>) -> Option<GlobalItemId> {
        self.get_global_type_id(&item.wrapper().as_any().type_id())
    }

    /// Get the [`GlobalItemId`] for the given [`TypeId`].
    ///
    /// Returns `None` if the item was not registered.
    #[must_use]
    pub fn get_global_type_id(&self, type_id: &TypeId) -> Option<GlobalItemId> {
        self.0.get_index_of(type_id)
    }

    /// Register an item type with the storage.
    ///
    /// This is required for converting between global ids and items.
    ///
    /// # Note
    /// The order in which items are registered is important.
    ///
    /// If an item is registered out of order, all following items will have
    /// their global ids shifted incorrectly.
    pub fn register<I: ItemTypeExt<V>>(&mut self) {
        self.0.store(TypeId::of::<I>(), I::as_static());
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: Version> Default for AppItemStorage<V>
where Vanilla: ItemResolver<V>
{
    #[inline]
    fn default() -> Self { Self::new() }
}

impl<V: Version> Default for ItemStorage<V>
where Vanilla: ItemResolver<V>
{
    fn default() -> Self { Self::new() }
}

// -------------------------------------------------------------------------------------------------

impl GlobalItemId {
    /// Create a new index with the given value.
    ///
    /// # Warning
    /// There is no guarantee that the given index is valid or represents the
    /// same index between versions. Indices may even change between program
    /// runs!
    #[inline]
    #[must_use]
    pub const fn new_unchecked_u32(index: u32) -> Self { Self(index) }
}

impl From<usize> for GlobalItemId {
    #[cfg(debug_assertions)]
    fn from(id: usize) -> Self { Self(u32::try_from(id).expect("GlobalItemId is too large!")) }

    #[inline]
    #[cfg(not(debug_assertions))]
    #[expect(clippy::cast_possible_truncation)]
    fn from(id: usize) -> Self { Self(id as u32) }
}

impl From<GlobalItemId> for usize {
    #[cfg(debug_assertions)]
    fn from(id: GlobalItemId) -> Self { usize::try_from(id.0).expect("GlobalItemId is too large!") }

    #[inline]
    #[cfg(not(debug_assertions))]
    fn from(id: GlobalItemId) -> Self { id.0 as usize }
}
