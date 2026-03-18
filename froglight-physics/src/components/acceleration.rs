use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use glam::Vec3;

/// Acceleration of an entity.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(PreviousAcceleration, super::Velocity))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct Acceleration(pub Vec3);

/// The [`Acceleration`] of the previous tick.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(super::PreviousVelocity))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct PreviousAcceleration(pub Vec3);

// -------------------------------------------------------------------------------------------------

macro_rules! impls {
    ($($ty:ident),*) => {
        $(
            impl AsRef<Vec3> for $ty {
                fn as_ref(&self) -> &Vec3 { &self.0 }
            }
            impl AsMut<Vec3> for $ty {
                fn as_mut(&mut self) -> &mut Vec3 { &mut self.0 }
            }

            impl Borrow<Vec3> for $ty {
                fn borrow(&self) -> &Vec3 { &self.0 }
            }
            impl BorrowMut<Vec3> for $ty {
                fn borrow_mut(&mut self) -> &mut Vec3 { &mut self.0 }
            }

            impl Deref for $ty {
                type Target = Vec3;

                fn deref(&self) -> &Self::Target { &self.0 }
            }
            impl DerefMut for $ty {
                fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
            }

            impl From<Vec3> for $ty {
                fn from(value: Vec3) -> Self { Self(value) }
            }
            impl From<$ty> for Vec3 {
                fn from(value: $ty) -> Self { value.0 }
            }
        )*
    };
}

impls!(Acceleration, PreviousAcceleration);
