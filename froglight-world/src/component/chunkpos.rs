#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use glam::IVec2;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A chunk's position in the world.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
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

    /// Get the x coordinate of this [`ChunkPos`].
    #[inline]
    #[must_use]
    pub const fn x(&self) -> i32 { self.0.x }

    /// Get the z coordinate of this [`ChunkPos`].
    #[inline]
    #[must_use]
    pub const fn z(&self) -> i32 { self.0.y }
}
