use core::ops::Deref;

use bevy_reflect::{FromType, Reflect};

use crate::menu::{MenuGroup, MenuGroupType};

/// A [`Reflect`] wrapper around [`MenuGroup`].
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
#[reflect(Debug, Clone, PartialEq, opaque)]
pub struct ReflectMenuGroup(&'static MenuGroup);

impl ReflectMenuGroup {
    /// Create a new [`ReflectMenuGroup`] of the given type.
    #[inline]
    #[must_use]
    pub fn new<G: MenuGroupType>() -> Self { Self(G::GROUP) }

    /// Get the inner [`MenuGroup`] reference.
    #[inline]
    #[must_use]
    pub const fn as_inner(&self) -> &'static MenuGroup { self.0 }
}

impl<G: MenuGroupType> FromType<G> for ReflectMenuGroup {
    #[inline]
    fn from_type() -> Self { Self::new::<G>() }
}

// -------------------------------------------------------------------------------------------------

impl Deref for ReflectMenuGroup {
    type Target = MenuGroup;

    fn deref(&self) -> &Self::Target { self.0 }
}
