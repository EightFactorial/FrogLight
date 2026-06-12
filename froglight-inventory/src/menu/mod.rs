//! TODO

use core::{any::TypeId, fmt};

#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;

mod global;
use froglight_common::identifier::Identifier;
pub use global::GlobalInventory;

/// Generic [`MenuGroupType`] data.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, opaque))]
pub struct MenuGroup {
    identifier: Identifier<'static>,
    type_id: TypeId,
}

impl MenuGroup {
    /// Create a new [`MenuGroup`] of the given type.
    #[inline]
    #[must_use]
    pub const fn new<G: MenuGroupType + ?Sized>() -> Self {
        Self { identifier: G::IDENTIFIER, type_id: TypeId::of::<G>() }
    }

    /// Get the [`Identifier`] of this group.
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier<'static> { &self.identifier }

    /// Get the [`TypeId`] of this group.
    #[inline]
    #[must_use]
    pub const fn type_id(&self) -> TypeId { self.type_id }
}

impl fmt::Debug for MenuGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MenuGroup").field(&self.identifier).finish()
    }
}

impl PartialEq for MenuGroup {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.type_id == other.type_id }
}
impl Eq for MenuGroup {}

// -------------------------------------------------------------------------------------------------

/// A trait for inventory menu groups.
pub trait MenuGroupType: 'static {
    /// A reference to this group's [`MenuGroup`].
    const GROUP: &'static MenuGroup;
    /// The group's unique identifier.
    const IDENTIFIER: Identifier<'static>;
}
