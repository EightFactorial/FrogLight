use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::Display;
#[cfg(feature = "io")]
use froglight_io::prelude::*;
use glam::IVec3;

/// A block position in the world.
#[repr(transparent)]
#[derive(Debug, Display, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq, Hash))]
pub struct BlockPos(IVec3);

impl BlockPos {
    /// Create a new [`BlockPos`] from the given coordinates.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::BlockPos;
    ///
    /// let block = BlockPos::new(0, 0, 0);
    /// assert_eq!(block.x(), 0);
    /// assert_eq!(block.y(), 0);
    /// assert_eq!(block.z(), 0);
    ///
    /// let block = BlockPos::new(1, 1, 1);
    /// assert_eq!(block.x(), 1);
    /// assert_eq!(block.y(), 1);
    /// assert_eq!(block.z(), 1);
    ///
    /// let block = BlockPos::new(-1000, 1000, -1000);
    /// assert_eq!(block.x(), -1000);
    /// assert_eq!(block.y(), 1000);
    /// assert_eq!(block.z(), -1000);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }

    /// Create a new [`BlockPos`] with all coordinates set to the same value.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::BlockPos;
    ///
    /// let block = BlockPos::splat(0);
    /// assert_eq!(block.x(), 0);
    /// assert_eq!(block.y(), 0);
    /// assert_eq!(block.z(), 0);
    ///
    /// let block = BlockPos::splat(1);
    /// assert_eq!(block.x(), 1);
    /// assert_eq!(block.y(), 1);
    /// assert_eq!(block.z(), 1);
    ///
    /// let block = BlockPos::splat(-1000);
    /// assert_eq!(block.x(), -1000);
    /// assert_eq!(block.y(), -1000);
    /// assert_eq!(block.z(), -1000);
    /// ```
    #[inline]
    #[must_use]
    pub const fn splat(v: i32) -> Self { Self(IVec3::splat(v)) }

    /// The x-coordinate of this block.
    #[inline]
    #[must_use]
    pub const fn x(&self) -> i32 { self.0.x }

    /// The y-coordinate of this block.
    #[inline]
    #[must_use]
    pub const fn y(&self) -> i32 { self.0.y }

    /// The z-coordinate of this block.
    #[inline]
    #[must_use]
    pub const fn z(&self) -> i32 { self.0.z }
}

// -------------------------------------------------------------------------------------------------

impl AsRef<IVec3> for BlockPos {
    fn as_ref(&self) -> &IVec3 { &self.0 }
}
impl AsMut<IVec3> for BlockPos {
    fn as_mut(&mut self) -> &mut IVec3 { &mut self.0 }
}

impl<T> From<T> for BlockPos
where IVec3: From<T>
{
    fn from(value: T) -> Self { Self(From::from(value)) }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl BlockPos {
    // <3 Azalea
    const PACKED_X_LENGTH: i64 = 1 + 25;
    const PACKED_X_MASK: i64 = (1 << Self::PACKED_X_LENGTH) - 1;
    const PACKED_Y_LENGTH: i64 = 64 - Self::PACKED_X_LENGTH - Self::PACKED_Z_LENGTH;
    const PACKED_Y_MASK: i64 = (1 << Self::PACKED_Y_LENGTH) - 1;
    const PACKED_Z_LENGTH: i64 = Self::PACKED_X_LENGTH;
    const PACKED_Z_MASK: i64 = (1 << Self::PACKED_Z_LENGTH) - 1;
    const X_OFFSET: i64 = Self::PACKED_Y_LENGTH + Self::PACKED_Z_LENGTH;
    const Z_OFFSET: i64 = Self::PACKED_Y_LENGTH;
}

#[cfg(feature = "io")]
impl FrogRead for BlockPos {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        let val = i64::frog_read(buffer)?;

        let x = (val << (64 - Self::X_OFFSET - Self::PACKED_X_LENGTH)
            >> (64 - Self::PACKED_X_LENGTH)) as i32;
        let y = (val << (64 - Self::PACKED_Y_LENGTH) >> (64 - Self::PACKED_Y_LENGTH)) as i32;
        let z = (val << (64 - Self::Z_OFFSET - Self::PACKED_Z_LENGTH)
            >> (64 - Self::PACKED_Z_LENGTH)) as i32;

        Ok(Self::new(x, y, z))
    }
}

#[cfg(feature = "io")]
impl FrogWrite for BlockPos {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        let mut val: i64 = 0;
        val |= (i64::from(self.x()) & Self::PACKED_X_MASK) << Self::X_OFFSET;
        val |= i64::from(self.y()) & Self::PACKED_Y_MASK;
        val |= (i64::from(self.z()) & Self::PACKED_Z_MASK) << Self::Z_OFFSET;
        val.frog_write(buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { i64::frog_len(&0) }
}

// -------------------------------------------------------------------------------------------------

impl<T> Add<T> for BlockPos
where IVec3: Add<T, Output = IVec3>
{
    type Output = Self;

    fn add(self, rhs: T) -> BlockPos { Self(self.0 + rhs) }
}
impl<T> AddAssign<T> for BlockPos
where IVec3: AddAssign<T>
{
    fn add_assign(&mut self, rhs: T) { self.0 += rhs; }
}

impl<T> Sub<T> for BlockPos
where IVec3: Sub<T, Output = IVec3>
{
    type Output = Self;

    fn sub(self, rhs: T) -> BlockPos { Self(self.0 - rhs) }
}
impl<T> SubAssign<T> for BlockPos
where IVec3: SubAssign<T>
{
    fn sub_assign(&mut self, rhs: T) { self.0 -= rhs; }
}

impl<T> Mul<T> for BlockPos
where IVec3: Mul<T, Output = IVec3>
{
    type Output = Self;

    fn mul(self, rhs: T) -> BlockPos { Self(self.0 * rhs) }
}
impl<T> MulAssign<T> for BlockPos
where IVec3: MulAssign<T>
{
    fn mul_assign(&mut self, rhs: T) { self.0 *= rhs; }
}

impl<T> Div<T> for BlockPos
where IVec3: Div<T, Output = IVec3>
{
    type Output = Self;

    fn div(self, rhs: T) -> BlockPos { Self(self.0 / rhs) }
}
impl<T> DivAssign<T> for BlockPos
where IVec3: DivAssign<T>
{
    fn div_assign(&mut self, rhs: T) { self.0 /= rhs; }
}
