use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

/// Whether the entity is on the ground.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(PreviousOnGround))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct OnGround(pub bool);

/// The [`OnGround`] of the previous tick.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct PreviousOnGround(pub bool);

// -------------------------------------------------------------------------------------------------

macro_rules! impls {
    ($($ty:ident),*) => {
        $(
            impl AsRef<bool> for $ty {
                fn as_ref(&self) -> &bool { &self.0 }
            }
            impl AsMut<bool> for $ty {
                fn as_mut(&mut self) -> &mut bool { &mut self.0 }
            }

            impl Borrow<bool> for $ty {
                fn borrow(&self) -> &bool { &self.0 }
            }
            impl BorrowMut<bool> for $ty {
                fn borrow_mut(&mut self) -> &mut bool { &mut self.0 }
            }

            impl Deref for $ty {
                type Target = bool;

                fn deref(&self) -> &Self::Target { &self.0 }
            }
            impl DerefMut for $ty {
                fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
            }

            impl From<bool> for $ty {
                fn from(value: bool) -> Self { Self(value) }
            }
            impl From<$ty> for bool {
                fn from(value: $ty) -> Self { value.0 }
            }
        )*
    };
}

impls!(OnGround, PreviousOnGround);
