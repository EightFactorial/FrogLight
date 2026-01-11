use core::ops::{Add, Div, Mul, Sub};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
use glam::IVec3;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{CHUNK_LENGTH, CHUNK_WIDTH, component::ChunkBlockPos, prelude::ChunkPos};

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

    /// Create a new [`BlockPos`] from the given [`ChunkBlockPos`],
    /// [`ChunkPos`], and vertical offset.
    #[must_use]
    pub const fn from_chunk_blockpos(block: ChunkBlockPos, chunk: ChunkPos, offset: i32) -> Self {
        Self::new(IVec3::new(
            block.x() as i32 + (chunk.x() * CHUNK_LENGTH as i32),
            block.y() as i32 + offset,
            block.z() as i32 + (chunk.z() * CHUNK_WIDTH as i32),
        ))
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: Into<IVec3>> From<T> for BlockPos {
    #[inline]
    fn from(value: T) -> Self { BlockPos::new(value.into()) }
}

impl Add<BlockPos> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn add(self, rhs: BlockPos) -> Self::Output { BlockPos::new(self.0 + rhs.0) }
}
impl Add<[i32; 3]> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn add(self, rhs: [i32; 3]) -> Self::Output { BlockPos::new(self.0 + IVec3::from(rhs)) }
}

impl Sub<BlockPos> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn sub(self, rhs: BlockPos) -> Self::Output { BlockPos::new(self.0 - rhs.0) }
}
impl Sub<[i32; 3]> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn sub(self, rhs: [i32; 3]) -> Self::Output { BlockPos::new(self.0 - IVec3::from(rhs)) }
}

impl Mul<BlockPos> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn mul(self, rhs: BlockPos) -> Self::Output { BlockPos::new(self.0 * rhs.0) }
}
impl Mul<[i32; 3]> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn mul(self, rhs: [i32; 3]) -> Self::Output { BlockPos::new(self.0 * IVec3::from(rhs)) }
}
impl Mul<i32> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output { BlockPos::new(self.0 * rhs) }
}

impl Div<BlockPos> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn div(self, rhs: BlockPos) -> Self::Output { BlockPos::new(self.0 / rhs.0) }
}
impl Div<[i32; 3]> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn div(self, rhs: [i32; 3]) -> Self::Output { BlockPos::new(self.0 / IVec3::from(rhs)) }
}
impl Div<i32> for BlockPos {
    type Output = BlockPos;

    #[inline]
    fn div(self, rhs: i32) -> Self::Output { BlockPos::new(self.0 / rhs) }
}
