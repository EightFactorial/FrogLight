use core::ops::Deref;

use bevy_reflect::{FromType, Reflect};

use crate::menu::{MenuGroup, MenuType};

/// A [`Reflect`] wrapper around [`MenuGroup`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, Clone, PartialEq, opaque)]
pub struct ReflectMenuGroup(&'static MenuGroup);

impl ReflectMenuGroup {
    /// Create a new [`ReflectMenuGroup`] of the given type.
    #[inline]
    #[must_use]
    pub const fn new<G: MenuType>() -> Self { Self(G::MENU_GROUP) }

    /// Get a reference to the inner [`MenuGroup`].
    #[inline]
    #[must_use]
    pub const fn as_inner(&self) -> &'static MenuGroup { self.0 }
}

impl<G: MenuType> FromType<G> for ReflectMenuGroup {
    #[inline]
    fn from_type() -> Self { Self::new::<G>() }
}

// -------------------------------------------------------------------------------------------------

impl Deref for ReflectMenuGroup {
    type Target = MenuGroup;

    fn deref(&self) -> &Self::Target { self.0 }
}
