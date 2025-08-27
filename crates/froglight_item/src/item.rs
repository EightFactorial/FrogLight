//! TODO

use core::{borrow::Borrow, fmt::Debug, ops::Deref};

use froglight_common::version::Version;

use crate::{
    info::{ItemComponentMap, ItemInfo},
    storage::Items,
};

/// A trait implemented by all item types, for each [`Version`] they exist.
pub trait ItemType<V: Version>: 'static {
    /// Get the [`ItemInfo`] for this item type.
    fn info() -> &'static ItemInfo;
}

// -------------------------------------------------------------------------------------------------

/// An item in the world.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Item {
    item_info: &'static ItemInfo,
}

impl Item {
    /// Create a new [`Item`] from an [`ItemType`] and [`Version`].
    #[inline]
    #[must_use]
    pub fn new<I: ItemType<V>, V: Version>() -> Self { Item { item_info: I::info() } }

    /// Create a new [`Item`] from an [`ItemInfo`].
    #[inline]
    #[must_use]
    pub const fn new_from(item_info: &'static ItemInfo) -> Self { Item { item_info } }

    /// Get the item's information.
    #[inline]
    #[must_use]
    pub const fn item_info(&self) -> &'static ItemInfo { self.item_info }

    /// Get the item's default components.
    #[inline]
    #[must_use]
    pub const fn default_components(&self) -> &ItemComponentMap {
        self.item_info.default_components()
    }

    /// Convert a [`Item`] into a [`GlobalItemId`].
    #[must_use]
    pub fn into_global(self) -> GlobalItemId { GlobalItemId { item_id: self.item_info.base_id() } }

    /// Returns `true` if this item is of the given type.
    #[inline]
    #[must_use]
    #[cfg(feature = "nightly")]
    pub fn is_item<B: ItemType<V>, V: Version>(&self) -> bool { self.item_info.is_item::<B, V>() }
}

impl Debug for Item {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Item({}, {})", self.item_info.identifier(), self.item_info.base_id())
    }
}

// -------------------------------------------------------------------------------------------------

/// A item's global id.
///
/// Contains the item's id relative to all items in the same [`Version`].
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalItemId {
    item_id: u32,
}

impl GlobalItemId {
    /// Convert the [`GlobalItemId`] into a [`Item`].
    #[must_use]
    pub fn into_item<V: Items>(self) -> Option<Item> { V::items().read_blocking().get_item(self) }

    /// Convert the [`GlobalItemId`] into a [`ItemInfo`].
    #[must_use]
    pub fn into_info<V: Items>(self) -> Option<&'static ItemInfo> {
        V::items().read_blocking().get_info(self)
    }
}

impl AsRef<u32> for GlobalItemId {
    fn as_ref(&self) -> &u32 { &self.item_id }
}
impl Borrow<u32> for GlobalItemId {
    fn borrow(&self) -> &u32 { &self.item_id }
}

impl From<u32> for GlobalItemId {
    fn from(item_id: u32) -> Self { GlobalItemId { item_id } }
}
impl From<GlobalItemId> for u32 {
    fn from(item_id: GlobalItemId) -> Self { item_id.item_id }
}

impl Debug for GlobalItemId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GlobalItemId({})", self.item_id)
    }
}

impl Deref for GlobalItemId {
    type Target = u32;

    fn deref(&self) -> &Self::Target { &self.item_id }
}
