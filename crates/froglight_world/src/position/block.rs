use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use glam::IVec3;

/// A block's position in the world.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq, Hash))]
pub struct BlockPosition(IVec3);

impl BlockPosition {
    /// Create a new [`BlockPosition`] from the given coordinates.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::BlockPosition;
    ///
    /// let block = BlockPosition::new(0, 0, 0);
    /// assert_eq!(block.x(), 0);
    /// assert_eq!(block.y(), 0);
    /// assert_eq!(block.z(), 0);
    ///
    /// let block = BlockPosition::new(1, 1, 1);
    /// assert_eq!(block.x(), 1);
    /// assert_eq!(block.y(), 1);
    /// assert_eq!(block.z(), 1);
    ///
    /// let block = BlockPosition::new(-1000, 1000, -1000);
    /// assert_eq!(block.x(), -1000);
    /// assert_eq!(block.y(), 1000);
    /// assert_eq!(block.z(), -1000);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }

    /// Create a new [`BlockPosition`] with all coordinates set to the same
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::BlockPosition;
    ///
    /// let block = BlockPosition::splat(0);
    /// assert_eq!(block.x(), 0);
    /// assert_eq!(block.y(), 0);
    /// assert_eq!(block.z(), 0);
    ///
    /// let block = BlockPosition::splat(1);
    /// assert_eq!(block.x(), 1);
    /// assert_eq!(block.y(), 1);
    /// assert_eq!(block.z(), 1);
    ///
    /// let block = BlockPosition::splat(-1000);
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

impl<T> Add<T> for BlockPosition
where
    IVec3: Add<T, Output = IVec3>,
{
    type Output = Self;

    fn add(self, rhs: T) -> BlockPosition { Self(self.0 + rhs) }
}
impl<T> AddAssign<T> for BlockPosition
where
    IVec3: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: T) { self.0 += rhs; }
}

impl<T> Sub<T> for BlockPosition
where
    IVec3: Sub<T, Output = IVec3>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> BlockPosition { Self(self.0 - rhs) }
}
impl<T> SubAssign<T> for BlockPosition
where
    IVec3: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: T) { self.0 -= rhs; }
}

impl<T> Mul<T> for BlockPosition
where
    IVec3: Mul<T, Output = IVec3>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> BlockPosition { Self(self.0 * rhs) }
}
impl<T> MulAssign<T> for BlockPosition
where
    IVec3: MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: T) { self.0 *= rhs; }
}

impl<T> Div<T> for BlockPosition
where
    IVec3: Div<T, Output = IVec3>,
{
    type Output = Self;

    fn div(self, rhs: T) -> BlockPosition { Self(self.0 / rhs) }
}
impl<T> DivAssign<T> for BlockPosition
where
    IVec3: DivAssign<T>,
{
    fn div_assign(&mut self, rhs: T) { self.0 /= rhs; }
}
