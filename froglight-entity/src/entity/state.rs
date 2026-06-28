use crate::entity::EntityBundle;

/// A unique identifier for an entity type,
/// relative to all entity type in the same version.
///
/// This only guarantees uniqueness if both entity types are, for example,
/// from [`V26_1`](froglight_common::prelude::V26_1).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalEntityId(u32);

impl GlobalEntityId {
    /// Create a new [`GlobalEntityId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u32) -> Self { GlobalEntityId(id) }

    /// Get the inner [`u32`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u32 { self.0 }
}

impl<T: Into<u32>> From<T> for GlobalEntityId {
    #[inline]
    fn from(value: T) -> Self { GlobalEntityId(value.into()) }
}
impl From<EntityBundle> for GlobalEntityId {
    #[inline]
    fn from(value: EntityBundle) -> Self { value.global_id() }
}

impl<T: PartialEq<u32>> PartialEq<T> for GlobalEntityId {
    #[inline]
    fn eq(&self, other: &T) -> bool { other.eq(&self.0) }
}
