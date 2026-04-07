#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

#[allow(unused_imports, reason = "May be used depending on features")]
use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(PreviousIsFalling))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct IsFalling {
    distance: f32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct PreviousIsFalling {
    distance: f32,
}

impl From<IsFalling> for PreviousIsFalling {
    #[inline]
    fn from(is_falling: IsFalling) -> Self { Self { distance: is_falling.distance } }
}

// -------------------------------------------------------------------------------------------------

macro_rules! impls {
    ($($ty:ident),*) => {
        $(
            impl $ty {
                /// Returns `true` if the entity is currently falling.
                #[inline]
                #[must_use]
                pub const fn is_falling(&self) -> bool { self.distance > 0.0 }

                /// Gets the distance the entity has fallen so far,
                /// or `0.0` if the entity is not falling.
                #[inline]
                #[must_use]
                pub const fn fall_distance(&self) -> f32 { self.distance }

                /// Sets the distance the entity has fallen.
                #[inline]
                pub const fn set_distance(&mut self, distance: f32) { self.distance = distance; }

                /// Adds to the distance the entity has fallen.
                #[inline]
                pub const fn add_distance(&mut self, distance: f32) { self.distance += distance; }
            }
        )*
    };
}

impls!(IsFalling, PreviousIsFalling);
