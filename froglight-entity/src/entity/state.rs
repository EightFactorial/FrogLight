use crate::entity::EntityBundle;

/// A unique identifier for an entity type, relative to all entity types in the
/// same version.
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

    /// Get the inner [`usize`] value.
    #[inline]
    #[must_use]
    pub const fn into_usize(self) -> usize { self.0 as usize }
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
