// use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub,
// SubAssign};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use glam::IVec2;

/// A [`Chunk`](crate::prelude::Chunk)'s position in the world.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, PartialEq, Hash, Component))]
pub struct ChunkPosition(IVec2);

impl ChunkPosition {
    /// Create a new [`ChunkPosition`].
    #[must_use]
    pub const fn new(position: IVec2) -> Self { Self(position) }

    /// Get the `x` coordinate of the chunk position.
    #[must_use]
    pub const fn x(&self) -> i32 { self.0.x }

    /// Get the `z` coordinate of the chunk position.
    #[must_use]
    pub const fn z(&self) -> i32 { self.0.y }
}

impl<T: Into<IVec2>> From<T> for ChunkPosition {
    #[inline]
    fn from(value: T) -> Self { Self::new(value.into()) }
}
