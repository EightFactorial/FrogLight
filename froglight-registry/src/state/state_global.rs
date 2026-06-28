// use crate::registry::Registry;

/// A unique identifier for a registry,
/// relative to all other item in the same version.
///
/// This only guarantees uniqueness if both registries are, for example,
/// from [`V26_1`](froglight_common::prelude::V26_1).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalRegistryId(u32);

impl GlobalRegistryId {
    /// Create a new [`GlobalRegistryId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u32) -> Self { GlobalRegistryId(id) }

    /// Get the inner [`u32`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u32 { self.0 }
}

// impl<T: Into<u32>> From<T> for GlobalRegistryId {
//     #[inline]
//     fn from(value: T) -> Self { GlobalRegistryId(value.into()) }
// }
// impl From<Item> for GlobalRegistryId {
//     #[inline]
//     fn from(value: Item) -> Self { value.global_id() }
// }

impl<T: PartialEq<u32>> PartialEq<T> for GlobalRegistryId {
    #[inline]
    fn eq(&self, other: &T) -> bool { other.eq(&self.0) }
}
