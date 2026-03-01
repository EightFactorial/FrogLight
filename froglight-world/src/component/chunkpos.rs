#![allow(
    clippy::unsafe_derive_deserialize,
    reason = "Triggered by deriving `Facet` and `Deserialize`"
)]

use core::ops::{Add, Div, Mul, Sub};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use glam::IVec2;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A chunk's position in the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", component(immutable))]
#[cfg_attr(feature = "facet", derive(facet::Facet), facet(opaque))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ChunkPos(IVec2);

impl ChunkPos {
    /// Create a new [`ChunkPos`] from the given coordinates.
    #[inline]
    #[must_use]
    pub const fn new(coords: IVec2) -> Self { Self(coords) }

    /// Create a new [`ChunkPos`] from the given x, y, and z coordinates.
    #[inline]
    #[must_use]
    pub const fn new_xz(x: i32, z: i32) -> Self { Self(IVec2::new(x, z)) }

    /// Create a new [`ChunkPos`] all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: i32) -> Self { Self(IVec2::splat(v)) }

    /// Get the x coordinate of this [`ChunkPos`].
    #[inline]
    #[must_use]
    pub const fn x(&self) -> i32 { self.0.x }

    /// Get the z coordinate of this [`ChunkPos`].
    #[inline]
    #[must_use]
    pub const fn z(&self) -> i32 { self.0.y }
}

// -------------------------------------------------------------------------------------------------

impl<T: Into<IVec2>> From<T> for ChunkPos {
    #[inline]
    fn from(value: T) -> Self { ChunkPos::new(value.into()) }
}

impl Add<ChunkPos> for ChunkPos {
    type Output = ChunkPos;

    #[inline]
    fn add(self, rhs: ChunkPos) -> Self::Output { ChunkPos::new(self.0 + rhs.0) }
}
impl Add<[i32; 2]> for ChunkPos {
    type Output = ChunkPos;

    #[inline]
    fn add(self, rhs: [i32; 2]) -> Self::Output { ChunkPos::new(self.0 + IVec2::from(rhs)) }
}

impl Sub<ChunkPos> for ChunkPos {
    type Output = ChunkPos;

    #[inline]
    fn sub(self, rhs: ChunkPos) -> Self::Output { ChunkPos::new(self.0 - rhs.0) }
}
impl Sub<[i32; 2]> for ChunkPos {
    type Output = ChunkPos;

    #[inline]
    fn sub(self, rhs: [i32; 2]) -> Self::Output { ChunkPos::new(self.0 - IVec2::from(rhs)) }
}

impl Mul<ChunkPos> for ChunkPos {
    type Output = ChunkPos;

    #[inline]
    fn mul(self, rhs: ChunkPos) -> Self::Output { ChunkPos::new(self.0 * rhs.0) }
}
impl Mul<[i32; 2]> for ChunkPos {
    type Output = ChunkPos;

    #[inline]
    fn mul(self, rhs: [i32; 2]) -> Self::Output { ChunkPos::new(self.0 * IVec2::from(rhs)) }
}
impl Mul<i32> for ChunkPos {
    type Output = ChunkPos;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output { ChunkPos::new(self.0 * rhs) }
}

impl Div<ChunkPos> for ChunkPos {
    type Output = ChunkPos;

    #[inline]
    fn div(self, rhs: ChunkPos) -> Self::Output { ChunkPos::new(self.0 / rhs.0) }
}
impl Div<[i32; 2]> for ChunkPos {
    type Output = ChunkPos;

    #[inline]
    fn div(self, rhs: [i32; 2]) -> Self::Output { ChunkPos::new(self.0 / IVec2::from(rhs)) }
}
impl Div<i32> for ChunkPos {
    type Output = ChunkPos;

    #[inline]
    fn div(self, rhs: i32) -> Self::Output { ChunkPos::new(self.0 / rhs) }
}
