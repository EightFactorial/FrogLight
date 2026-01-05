#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use glam::IVec3;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A block's position in the world.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct BlockPos(IVec3);

impl BlockPos {
    /// Create a new [`BlockPos`] from the given coordinates.
    #[inline]
    #[must_use]
    pub const fn new(coords: IVec3) -> Self { Self(coords) }

    /// Create a new [`BlockPos`] from the given x, y, and z coordinates.
    #[inline]
    #[must_use]
    pub const fn new_xyz(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }

    /// Get the x coordinate of this [`BlockPos`].
    #[inline]
    #[must_use]
    pub const fn x(&self) -> i32 { self.0.x }

    /// Get the y coordinate of this [`BlockPos`].
    #[inline]
    #[must_use]
    pub const fn y(&self) -> i32 { self.0.y }

    /// Get the z coordinate of this [`BlockPos`].
    #[inline]
    #[must_use]
    pub const fn z(&self) -> i32 { self.0.z }
}

// -------------------------------------------------------------------------------------------------
