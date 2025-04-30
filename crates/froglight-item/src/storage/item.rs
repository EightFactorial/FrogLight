use alloc::sync::Arc;
use core::any::TypeId;

#[cfg(all(feature = "bevy", feature = "reflect"))]
use bevy_ecs::reflect::ReflectResource;
#[cfg(feature = "bevy")]
use bevy_ecs::resource::Resource;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;
use downcast_rs::Downcast;
use froglight_common::{vanilla::Vanilla, version::Version};
use froglight_nbt::nbt::UnnamedNbt;
use indexmap::IndexMap;
use parking_lot::RwLock;

use super::GlobalItemId;
use crate::{
    item::{ItemType, ItemTypeExt, UntypedItem},
    resolve::ItemResolver,
};

/// A thread-safe dynamic storage for item types.
///
/// Allows for the registration and retrieval of item types at runtime.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Resource))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(all(feature = "bevy", feature = "reflect"), reflect(Clone, Resource))]
pub struct AppItemStorage<V: Version>(Arc<RwLock<ItemStorage<V>>>);

impl<V: Version> Default for AppItemStorage<V>
where Vanilla: ItemResolver<V>
{
    #[inline]
    fn default() -> Self { Self::new() }
}
impl<V: Version> core::ops::Deref for AppItemStorage<V> {
    type Target = Arc<RwLock<ItemStorage<V>>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<V: Version> AppItemStorage<V> {
    /// Create a new [`AppItemStorage`] with the [`Vanilla`] types registered.
    #[inline]
    #[must_use]
    pub fn new() -> Self
    where Vanilla: ItemResolver<V> {
        Self::from_storage(ItemStorage::new())
    }

    /// Create a new [`AppItemStorage`] from a [`ItemStorage`].
    #[inline]
    #[must_use]
    pub fn from_storage(storage: ItemStorage<V>) -> Self { Self(Arc::new(RwLock::new(storage))) }
}

// -------------------------------------------------------------------------------------------------

/// A dynamic storage for item types.
///
/// Allows for the registration and retrieval of item types at runtime.
pub struct ItemStorage<V: Version>(IndexMap<TypeId, ItemWrapper<V>>);

impl<V: Version> Default for ItemStorage<V>
where Vanilla: ItemResolver<V>
{
    fn default() -> Self { Self::new() }
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
    #[inline]
    #[must_use]
    pub fn new_empty() -> Self { Self(IndexMap::default()) }

    /// Get the [`ItemType`] for the given [`GlobalItemId`].
    ///
    /// Handy for storing many item types and bulk operations.
    ///
    /// Returns `None` if no item with the given id was registered.
    #[must_use]
    pub fn get_trait(&self, item: GlobalItemId) -> Option<&'static dyn ItemType<V>> {
        self.0.get_index((*item) as usize).map(|(_, wrapper)| **wrapper)
    }

    /// Get the [`UntypedItem`] for the given [`GlobalItemId`].
    ///
    /// If no data is provided, the item's default NBT data will be used.
    ///
    /// Returns `None` if no item with the given id was registered.
    #[must_use]
    pub fn get_untyped(
        &self,
        item: GlobalItemId,
        data: Option<UnnamedNbt>,
    ) -> Option<UntypedItem<V>> {
        self.0.get_index(*item as usize).map(|(_, wrapper)| {
            UntypedItem::new(data.unwrap_or_else(|| wrapper.default_nbt()), *wrapper)
        })
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
        item: GlobalItemId,
        data: Option<UnnamedNbt>,
    ) -> Option<Result<R::ItemEnum, UntypedItem<V>>> {
        self.get_untyped(item, data).map(R::resolve)
    }

    /// Get the [`GlobalItemId`] for the given item.
    ///
    /// Returns `None` if the item was not registered.
    #[must_use]
    #[expect(clippy::cast_possible_truncation)]
    pub fn get_global(&self, item: &UntypedItem<V>) -> Option<GlobalItemId> {
        self.0
            .get_index_of(&<dyn ItemType<V> as Downcast>::as_any(**item.wrapper()).type_id())
            .map(|index| GlobalItemId::new_unchecked(index as u32))
    }

    /// Get the [`GlobalItemId`] for the given [`ItemType`].
    ///
    /// Returns `None` if the item was not registered.
    #[must_use]
    #[expect(clippy::cast_possible_truncation)]
    pub fn get_global_type<I: ItemType<V>>(&self) -> Option<GlobalItemId> {
        self.0
            .get_index_of(&TypeId::of::<I>())
            .map(|index| GlobalItemId::new_unchecked(index as u32))
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
        self.0.insert(TypeId::of::<I>(), ItemWrapper::new(I::as_static()));
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around a [`&'static dyn ItemType`](ItemType)
/// that implements [`PartialEq`] and [`Eq`].
#[derive(Clone, Copy)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect), reflect(Clone, PartialEq))]
pub(crate) struct ItemWrapper<V: Version>(&'static dyn ItemType<V>);

impl<V: Version> ItemWrapper<V> {
    /// Create a new [`ItemWrapper`] from the given item type.
    #[inline]
    #[must_use]
    pub(crate) const fn new(item: &'static dyn ItemType<V>) -> Self { Self(item) }
}

impl<V: Version> Eq for ItemWrapper<V> {}
impl<V: Version> PartialEq for ItemWrapper<V> {
    fn eq(&self, other: &Self) -> bool {
        <dyn ItemType<V> as Downcast>::as_any(self.0).type_id()
            == <dyn ItemType<V> as Downcast>::as_any(other.0).type_id()
    }
}
impl<V: Version> core::ops::Deref for ItemWrapper<V> {
    type Target = &'static dyn ItemType<V>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
