#[cfg(feature = "facet")]
use facet_minecraft as mc;

use crate::entity::EntityBundle;

/// A unique identifier for an entity type,
/// relative to all other entity types.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalId(u32);

impl GlobalId {
    /// Create a new [`GlobalId`].
    #[inline]
    #[must_use]
    pub const fn new(id: u32) -> Self { GlobalId(id) }

    /// Get the inner [`u32`] value.
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> u32 { self.0 }
}

impl<T: Into<u32>> From<T> for GlobalId {
    fn from(value: T) -> Self { GlobalId(value.into()) }
}
impl From<EntityBundle> for GlobalId {
    fn from(value: EntityBundle) -> Self { value.global_id() }
}

impl<T: PartialEq<u32>> PartialEq<T> for GlobalId {
    fn eq(&self, other: &T) -> bool { other.eq(&self.0) }
}

// -------------------------------------------------------------------------------------------------

/// A variable-length [`u32`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct VarInt(#[cfg_attr(feature = "facet", facet(mc::variable))] pub i32);

/// A variable-length [`u64`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct VarLong(#[cfg_attr(feature = "facet", facet(mc::variable))] pub i32);
