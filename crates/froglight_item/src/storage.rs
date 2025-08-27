//! TODO

use alloc::vec::Vec;
use core::ops::Deref;

use froglight_common::version::Version;

use crate::{
    info::ItemInfo,
    item::{GlobalItemId, Item, ItemType},
};

/// A [`Version`] with an associated [`ItemMap`].
pub trait Items: Version {
    /// Get the [`StaticItemMap`] for this [`Version`].
    fn items() -> &'static StaticItemMap;
    /// Initialize this version's items into the provided [`ItemMap`].
    fn init_items(map: &mut ItemMap);
}

// -------------------------------------------------------------------------------------------------

/// A modifiable, thread-safe reference to a [`ItemMap`].
#[repr(transparent)]
pub struct StaticItemMap(
    #[cfg(feature = "async")] async_lock::RwLock<ItemMap>,
    #[cfg(not(feature = "async"))] parking_lot::RwLock<ItemMap>,
);

impl StaticItemMap {
    /// Create a new [`StaticItemMap`].
    #[must_use]
    #[cfg(feature = "async")]
    pub const fn new(info: ItemMap) -> Self { StaticItemMap(async_lock::RwLock::new(info)) }

    /// Read the [`ItemMap`], blocking the current thread if necessary.
    #[must_use]
    #[cfg(feature = "async")]
    pub fn read_blocking(&self) -> async_lock::RwLockReadGuard<'_, ItemMap> {
        self.0.read_blocking()
    }

    /// Create a new [`StaticItemMap`].
    #[must_use]
    #[cfg(not(feature = "async"))]
    pub const fn new(info: ItemMap) -> Self { StaticItemMap(parking_lot::RwLock::new(info)) }

    /// Read the [`ItemMap`], blocking the current thread if necessary.
    #[cfg(not(feature = "async"))]
    pub fn read_blocking(&self) -> parking_lot::RwLockReadGuard<'_, ItemMap> { self.0.read() }
}

impl Deref for StaticItemMap {
    #[cfg(feature = "async")]
    type Target = async_lock::RwLock<ItemMap>;
    #[cfg(not(feature = "async"))]
    type Target = parking_lot::RwLock<ItemMap>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

// -------------------------------------------------------------------------------------------------

/// A list of static [`ItemInfo`]s.
///
/// Used for assigning ids to items and retrieving their information.
pub struct ItemMap(Vec<&'static ItemInfo>);

impl ItemMap {
    /// Create a new empty [`ItemMap`].
    #[must_use]
    pub const fn new_empty() -> Self { ItemMap(Vec::new()) }

    /// Initialize the [`ItemMap`] with items from the given version.
    #[inline]
    pub fn init<V: Items>(&mut self) { V::init_items(self); }

    /// Get a [`Item`] for a given [`GlobalItemId`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemMap`].
    #[must_use]
    pub fn get_item(&self, item: GlobalItemId) -> Option<Item> {
        self.get_info(item).map(Item::new_from)
    }

    /// Get the [`ItemInfo`] for a given [`GlobalItemId`].
    ///
    /// Returns `None` if the item is not registered in the [`ItemMap`].
    #[must_use]
    pub fn get_info(&self, item: GlobalItemId) -> Option<&'static ItemInfo> {
        self.0.get(*item as usize).copied()
    }

    /// Get the number of items registered in this [`ItemMap`].
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the [`ItemMap`] is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Register a [`ItemType`] in the [`ItemMap`].
    ///
    /// Assigns a [`GlobalItemId`] to the [`ItemType`].
    #[inline]
    pub fn register<B: ItemType<V>, V: Version>(&mut self) { self.register_untyped(B::info()); }

    /// Register a [`ItemType`] in the [`ItemMap`].
    ///
    /// Assigns a [`GlobalItemId`] to the [`ItemType`].
    #[expect(clippy::cast_possible_truncation, reason = "There will never be 4,294,967,295 items")]
    pub fn register_untyped(&mut self, info: &'static ItemInfo) {
        info.set_registered_id(self.0.len() as u32);
        self.0.push(info);
    }

    /// Get a reference to the inner [`Vec`] of the [`ItemMap`].
    ///
    /// Requires calling [`ItemMap::as_inner`] explicitly.
    #[inline]
    #[must_use]
    pub fn as_inner(map: &Self) -> &Vec<&'static ItemInfo> { &map.0 }

    /// Get a mutable reference to the inner [`Vec`] of the [`ItemMap`].
    ///
    /// Requires calling [`ItemMap::as_inner_mut`] explicitly.
    #[inline]
    #[must_use]
    pub fn as_inner_mut(map: &mut Self) -> &mut Vec<&'static ItemInfo> { &mut map.0 }
}
