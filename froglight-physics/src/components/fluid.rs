#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

#[allow(unused_imports, reason = "May be used depending on features")]
use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", require(PreviousInFluid))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct InFluid {
    water_height: f32,
    lava_height: f32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct PreviousInFluid {
    water_height: f32,
    lava_height: f32,
}

impl From<InFluid> for PreviousInFluid {
    #[inline]
    fn from(in_fluid: InFluid) -> Self {
        Self { water_height: in_fluid.water_height, lava_height: in_fluid.lava_height }
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! impls {
    ($($ty:ident),*) => {
        $(
            impl $ty {
                /// Returns `true` if the entity is currently in water.
                #[inline]
                #[must_use]
                pub const fn in_water(&self) -> bool { self.water_height > 0.0 }

                /// Returns `true` if the entity is currently in lava.
                #[inline]
                #[must_use]
                pub const fn in_lava(&self) -> bool { self.lava_height > 0.0 }

                /// Returns `true` if the entity is currently in any fluid.
                #[inline]
                #[must_use]
                pub const fn in_fluid(&self) -> bool { self.in_water() || self.in_lava() }

                /// Get the height of the water the entity is currently in,
                /// or `0.0` if not in water.
                #[inline]
                #[must_use]
                pub const fn water_height(&self) -> f32 { self.water_height }

                /// Get the height of the lava the entity is currently in,
                /// or `0.0` if not in lava.
                #[inline]
                #[must_use]
                pub const fn lava_height(&self) -> f32 { self.lava_height }

                /// Set the height of the water the entity is currently in.
                #[inline]
                pub const fn set_water_height(&mut self, height: f32) { self.water_height = height; }

                /// Set the height of the lava the entity is currently in.
                #[inline]
                pub const fn set_lava_height(&mut self, height: f32) { self.lava_height = height; }
            }
        )*
    };
}

impls!(InFluid, PreviousInFluid);
