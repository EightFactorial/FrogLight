use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::Display;
#[cfg(feature = "io")]
use froglight_io::prelude::*;
use glam::IVec2;

use super::BlockPos;

/// A chunk position in the world.
#[repr(transparent)]
#[derive(Debug, Display, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect, Component))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, PartialEq, Hash, Component))]
pub struct ChunkPos(IVec2);

impl ChunkPos {
    /// Create a new [`ChunkPos`] from the given coordinates.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::ChunkPos;
    ///
    /// let chunk = ChunkPos::new(0, 0);
    /// assert_eq!(chunk.x(), 0);
    /// assert_eq!(chunk.z(), 0);
    ///
    /// let chunk = ChunkPos::new(1, 1);
    /// assert_eq!(chunk.x(), 1);
    /// assert_eq!(chunk.z(), 1);
    ///
    /// let chunk = ChunkPos::new(-1000, 1000);
    /// assert_eq!(chunk.x(), -1000);
    /// assert_eq!(chunk.z(), 1000);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(x: i32, z: i32) -> Self { Self(IVec2::new(x, z)) }

    /// Create a new [`ChunkPos`] with all coordinates set to the same value.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::ChunkPos;
    ///
    /// let chunk = ChunkPos::splat(0);
    /// assert_eq!(chunk.x(), 0);
    /// assert_eq!(chunk.z(), 0);
    ///
    /// let chunk = ChunkPos::splat(1);
    /// assert_eq!(chunk.x(), 1);
    /// assert_eq!(chunk.z(), 1);
    ///
    /// let chunk = ChunkPos::splat(-1000);
    /// assert_eq!(chunk.x(), -1000);
    /// assert_eq!(chunk.z(), -1000);
    /// ```
    #[inline]
    #[must_use]
    pub const fn splat(v: i32) -> Self { Self(IVec2::splat(v)) }

    /// The x-coordinate of this chunk.
    #[inline]
    #[must_use]
    pub const fn x(&self) -> i32 { self.0.x }

    /// The z-coordinate of this chunk.
    #[inline]
    #[must_use]
    pub const fn z(&self) -> i32 { self.0.y }

    /// Create a [`ChunkPos`] from the given [`BlockPos`].
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::{BlockPos, ChunkPos};
    ///
    /// let block = BlockPos::new(0, 0, 0);
    /// assert_eq!(ChunkPos::from_block(block), ChunkPos::new(0, 0));
    ///
    /// let block = BlockPos::new(1, 0, 1);
    /// assert_eq!(ChunkPos::from_block(block), ChunkPos::new(0, 0));
    ///
    /// let block = BlockPos::new(16, 0, 16);
    /// assert_eq!(ChunkPos::from_block(block), ChunkPos::new(1, 1));
    ///
    /// let block = BlockPos::new(-1, 0, 0);
    /// assert_eq!(ChunkPos::from_block(block), ChunkPos::new(-1, 0));
    ///
    /// let block = BlockPos::new(0, 0, -1);
    /// assert_eq!(ChunkPos::from_block(block), ChunkPos::new(0, -1));
    ///
    /// let block = BlockPos::new(-16, 0, -16);
    /// assert_eq!(ChunkPos::from_block(block), ChunkPos::new(-1, -1));
    ///
    /// let block = BlockPos::new(-17, 0, -17);
    /// assert_eq!(ChunkPos::from_block(block), ChunkPos::new(-2, -2));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_block(block: BlockPos) -> Self {
        Self::new(block.x().div_euclid(16), block.z().div_euclid(16))
    }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<IVec2> for ChunkPos {
    fn as_ref(&self) -> &IVec2 { &self.0 }
}
impl AsMut<IVec2> for ChunkPos {
    fn as_mut(&mut self) -> &mut IVec2 { &mut self.0 }
}

impl<T> From<T> for ChunkPos
where IVec2: From<T>
{
    fn from(value: T) -> Self { Self(From::from(value)) }
}

impl From<BlockPos> for ChunkPos {
    #[inline]
    fn from(value: BlockPos) -> Self { Self::from_block(value) }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl FrogRead for ChunkPos {
    #[inline]
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        // Note: The x and z(y) coordinates are swapped here.
        let y = i32::frog_read(buffer)?;
        let x = i32::frog_read(buffer)?;
        Ok(Self::new(x, y))
    }
}
#[cfg(feature = "io")]
impl FrogWrite for ChunkPos {
    #[inline]
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        // Note: The x and z(y) coordinates are swapped here.
        self.0.y.frog_write(buffer)?;
        self.0.x.frog_write(buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { self.0.frog_len() }
}

#[cfg(feature = "io")]
impl FrogVarRead for ChunkPos {
    #[inline]
    fn frog_var_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        // Note: The x and z(y) coordinates are swapped here.
        let y = i32::frog_var_read(buffer)?;
        let x = i32::frog_var_read(buffer)?;
        Ok(Self::new(x, y))
    }
}
#[cfg(feature = "io")]
impl FrogVarWrite for ChunkPos {
    #[inline]
    fn frog_var_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        // Note: The x and z(y) coordinates are swapped here.
        self.0.y.frog_var_write(buffer)?;
        self.0.x.frog_var_write(buffer)
    }

    #[inline]
    fn frog_var_len(&self) -> usize { self.0.frog_var_len() }
}

// -------------------------------------------------------------------------------------------------

impl<T> Add<T> for ChunkPos
where IVec2: Add<T, Output = IVec2>
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output { Self(self.0 + rhs) }
}
impl<T> AddAssign<T> for ChunkPos
where IVec2: AddAssign<T>
{
    fn add_assign(&mut self, rhs: T) { self.0 += rhs; }
}

impl<T> Sub<T> for ChunkPos
where IVec2: Sub<T, Output = IVec2>
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output { Self(self.0 - rhs) }
}
impl<T> SubAssign<T> for ChunkPos
where IVec2: SubAssign<T>
{
    fn sub_assign(&mut self, rhs: T) { self.0 -= rhs; }
}

impl<T> Mul<T> for ChunkPos
where IVec2: Mul<T, Output = IVec2>
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output { Self(self.0 * rhs) }
}
impl<T> MulAssign<T> for ChunkPos
where IVec2: MulAssign<T>
{
    fn mul_assign(&mut self, rhs: T) { self.0 *= rhs; }
}

impl<T> Div<T> for ChunkPos
where IVec2: Div<T, Output = IVec2>
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output { Self(self.0 / rhs) }
}
impl<T> DivAssign<T> for ChunkPos
where IVec2: DivAssign<T>
{
    fn div_assign(&mut self, rhs: T) { self.0 /= rhs; }
}
