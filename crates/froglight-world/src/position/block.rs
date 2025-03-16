use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use glam::IVec3;

/// A block position in the world.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_io::prelude::FrogBuf))]
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
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }

    /// The x-coordinate of this block.
    #[must_use]
    pub const fn x(&self) -> i32 { self.0.x }

    /// The y-coordinate of this block.
    #[must_use]
    pub const fn y(&self) -> i32 { self.0.y }

    /// The z-coordinate of this block.
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
impl Into<IVec3> for BlockPos {
    fn into(self) -> IVec3 { self.0 }
}

// -------------------------------------------------------------------------------------------------

impl<T> Add<T> for BlockPos
where IVec3: Add<T, Output = IVec3>
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output { Self(self.0 + rhs) }
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

    fn sub(self, rhs: T) -> Self::Output { Self(self.0 - rhs) }
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

    fn mul(self, rhs: T) -> Self::Output { Self(self.0 * rhs) }
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

    fn div(self, rhs: T) -> Self::Output { Self(self.0 / rhs) }
}
impl<T> DivAssign<T> for BlockPos
where IVec3: DivAssign<T>
{
    fn div_assign(&mut self, rhs: T) { self.0 /= rhs; }
}
