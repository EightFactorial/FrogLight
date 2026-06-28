use crate::item::Item;

/// A unique identifier for a item,
/// relative to all other item in the same version.
///
/// This only guarantees uniqueness if both item are, for example,
/// from [`V26_1`](froglight_common::prelude::V26_1).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalItemId(u16);

impl GlobalItemId {
    /// Create a new [`GlobalItemId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u16) -> Self { GlobalItemId(id) }

    /// Get the inner [`u16`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u16 { self.0 }
}

impl<T: Into<u16>> From<T> for GlobalItemId {
    #[inline]
    fn from(value: T) -> Self { GlobalItemId(value.into()) }
}
impl From<Item> for GlobalItemId {
    #[inline]
    fn from(value: Item) -> Self { value.global_id() }
}

impl<T: PartialEq<u16>> PartialEq<T> for GlobalItemId {
    #[inline]
    fn eq(&self, other: &T) -> bool { other.eq(&self.0) }
}
