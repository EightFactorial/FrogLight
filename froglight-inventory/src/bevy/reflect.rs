use core::ops::Deref;

use bevy_reflect::{FromType, Reflect};

use crate::menu::{MenuGroup, MenuGroupType};

/// A [`Reflect`] wrapper around [`MenuGroup`].
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
#[reflect(Debug, Clone, PartialEq, opaque)]
pub struct ReflectMenuGroup(MenuGroup);

impl ReflectMenuGroup {
    /// Create a new [`ReflectMenuGroup`] of the given type.
    #[inline]
    #[must_use]
    pub const fn new<G: MenuGroupType>() -> Self { Self(MenuGroup::new::<G>()) }

    /// Get a reference to the inner [`MenuGroup`].
    #[inline]
    #[must_use]
    pub const fn as_inner(&self) -> &MenuGroup { &self.0 }

    /// Get the inner [`MenuGroup`].
    #[inline]
    #[must_use]
    pub const fn into_inner(self) -> MenuGroup { self.0 }
}

impl<G: MenuGroupType> FromType<G> for ReflectMenuGroup {
    #[inline]
    fn from_type() -> Self { Self::new::<G>() }
}

// -------------------------------------------------------------------------------------------------

impl Deref for ReflectMenuGroup {
    type Target = MenuGroup;

    fn deref(&self) -> &Self::Target { &self.0 }
}
