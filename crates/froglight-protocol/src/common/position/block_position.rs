use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use derive_more::{Deref, DerefMut, From, Into};
use glam::{DVec3, I64Vec3, IVec3, Vec3};

use crate::protocol::{FrogRead, FrogWrite, ReadError, WriteError};

/// A position in the world, measured in blocks.
///
/// # Example
/// ```rust
/// use froglight_protocol::common::BlockPosition;
///
/// let mut origin = BlockPosition::default();
///
/// // The default block position is (0, 0, 0).
/// assert_eq!(origin, BlockPosition::new(0, 0, 0));
///
/// // Add two block positions together.
/// origin += BlockPosition::new(1, 0, 0);
/// assert_eq!(origin, BlockPosition::new(1, 0, 0));
///
/// // Subtract two block positions.
/// origin -= BlockPosition::new(0, 0, 1);
/// assert_eq!(origin, BlockPosition::new(1, 0, -1));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, From, Into, Deref, DerefMut)]
// #[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0,
// 0])]
pub struct BlockPosition(I64Vec3);

impl BlockPosition {
    /// All zeros.
    pub const ZERO: Self = Self(I64Vec3::ZERO);

    /// All ones.
    pub const ONE: Self = Self(I64Vec3::ONE);

    /// All negative ones.
    pub const NEG_ONE: Self = Self(I64Vec3::NEG_ONE);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self(I64Vec3::X);
    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self(I64Vec3::Y);
    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self(I64Vec3::Z);

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self(I64Vec3::NEG_X);
    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self(I64Vec3::NEG_Y);
    /// A unit vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self(I64Vec3::NEG_Z);

    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    /// All `i64::MIN`.
    pub const MIN: Self = Self(I64Vec3::MIN);

    /// All `i64::MAX`.
    pub const MAX: Self = Self(I64Vec3::MAX);

    /// Create a new block position.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BlockPosition;
    ///
    /// let pos = BlockPosition::new(1, 2, 3);
    /// assert_eq!(pos.x(), 1);
    /// assert_eq!(pos.y(), 2);
    /// assert_eq!(pos.z(), 3);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(x: i64, y: i64, z: i64) -> Self { Self(I64Vec3::new(x, y, z)) }

    /// Create a new block position with `i32` values.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BlockPosition;
    ///
    /// let pos = BlockPosition::new_i32(1, 2, 3);
    /// assert_eq!(pos.x(), 1);
    /// assert_eq!(pos.y(), 2);
    /// assert_eq!(pos.z(), 3);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new_i32(x: i32, y: i32, z: i32) -> Self { Self::new(x as i64, y as i64, z as i64) }

    /// Creates a new [`BlockPosition`] where all coordinates are the same.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BlockPosition;
    ///
    /// let pos = BlockPosition::splat(0);
    /// assert_eq!(pos, BlockPosition::new(0, 0, 0));
    ///
    /// let pos = BlockPosition::splat(1);
    /// assert_eq!(pos, BlockPosition::new(1, 1, 1));
    ///
    /// let pos = BlockPosition::splat(-64);
    /// assert_eq!(pos, BlockPosition::new(-64, -64, -64));
    /// ```
    #[must_use]
    #[inline]
    pub const fn splat(v: i64) -> Self { Self(I64Vec3::splat(v)) }

    /// Creates a new [`BlockPosition`] where all coordinates are the same.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BlockPosition;
    ///
    /// let pos = BlockPosition::splat_i32(0);
    /// assert_eq!(pos, BlockPosition::new(0, 0, 0));
    ///
    /// let pos = BlockPosition::splat_i32(1);
    /// assert_eq!(pos, BlockPosition::new(1, 1, 1));
    ///
    /// let pos = BlockPosition::splat_i32(-64);
    /// assert_eq!(pos, BlockPosition::new(-64, -64, -64));
    /// ```
    #[must_use]
    #[inline]
    pub const fn splat_i32(v: i32) -> Self { Self::splat(v as i64) }

    /// Inverts all coordinates.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BlockPosition;
    ///
    /// assert_eq!(BlockPosition::ZERO.invert(), BlockPosition::ZERO);
    /// assert_eq!(BlockPosition::ONE.invert(), BlockPosition::NEG_ONE);
    ///
    /// assert_eq!(BlockPosition::new(1, 2, 3).invert(), BlockPosition::new(-1, -2, -3));
    /// ```
    #[must_use]
    pub const fn invert(self) -> Self { Self::new(-self.x(), -self.y(), -self.z()) }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance_squared(self, other: Self) -> i64 { self.0.distance_squared(other.0) }

    /// Gets the x-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn x(&self) -> i64 { self.0.x }
    /// Gets the y-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn y(&self) -> i64 { self.0.y }
    /// Gets the z-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn z(&self) -> i64 { self.0.z }
}

impl BlockPosition {
    // <3 Azalea
    const PACKED_X_LENGTH: i64 = 1 + 25;
    const PACKED_Z_LENGTH: i64 = Self::PACKED_X_LENGTH;
    const PACKED_Y_LENGTH: i64 = 64 - Self::PACKED_X_LENGTH - Self::PACKED_Z_LENGTH;
    const PACKED_X_MASK: i64 = (1 << Self::PACKED_X_LENGTH) - 1;
    const PACKED_Y_MASK: i64 = (1 << Self::PACKED_Y_LENGTH) - 1;
    const PACKED_Z_MASK: i64 = (1 << Self::PACKED_Z_LENGTH) - 1;
    const Z_OFFSET: i64 = Self::PACKED_Y_LENGTH;
    const X_OFFSET: i64 = Self::PACKED_Y_LENGTH + Self::PACKED_Z_LENGTH;
}

impl FrogRead for BlockPosition {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let val = i64::fg_read(buf)?;
        Ok(Self::new(
            val << (64 - Self::X_OFFSET - Self::PACKED_X_LENGTH) >> (64 - Self::PACKED_X_LENGTH),
            val << (64 - Self::PACKED_Y_LENGTH) >> (64 - Self::PACKED_Y_LENGTH),
            val << (64 - Self::Z_OFFSET - Self::PACKED_Z_LENGTH) >> (64 - Self::PACKED_Z_LENGTH),
        ))
    }
}

impl FrogWrite for BlockPosition {
    #[allow(clippy::cast_sign_loss)]
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        let mut val: i64 = 0;
        val |= (self.x & Self::PACKED_X_MASK) << Self::X_OFFSET;
        val |= self.y & Self::PACKED_Y_MASK;
        val |= (self.z & Self::PACKED_Z_MASK) << Self::Z_OFFSET;
        val.fg_write(buf)
    }
}

impl Add<BlockPosition> for BlockPosition {
    type Output = Self;
    fn add(self, rhs: BlockPosition) -> Self::Output {
        Self::new(self.x.add(rhs.x), self.y.add(rhs.y), self.z.add(rhs.z))
    }
}

impl AddAssign<BlockPosition> for BlockPosition {
    #[inline]
    fn add_assign(&mut self, rhs: BlockPosition) { self.0.add_assign(rhs.0) }
}

impl Sub<BlockPosition> for BlockPosition {
    type Output = Self;
    fn sub(self, rhs: BlockPosition) -> Self::Output {
        Self::new(self.x.sub(rhs.x), self.y.sub(rhs.y), self.z.sub(rhs.z))
    }
}

impl SubAssign<BlockPosition> for BlockPosition {
    #[inline]
    fn sub_assign(&mut self, rhs: BlockPosition) { self.0.sub_assign(rhs.0) }
}

impl Mul<BlockPosition> for BlockPosition {
    type Output = Self;
    fn mul(self, rhs: BlockPosition) -> Self::Output {
        Self::new(self.x.mul(rhs.x), self.y.mul(rhs.y), self.z.mul(rhs.z))
    }
}

impl MulAssign<BlockPosition> for BlockPosition {
    #[inline]
    fn mul_assign(&mut self, rhs: BlockPosition) { self.0.mul_assign(rhs.0) }
}

impl Mul<i64> for BlockPosition {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Self::new(self.x.mul(rhs), self.y.mul(rhs), self.z.mul(rhs))
    }
}

impl MulAssign<i64> for BlockPosition {
    #[inline]
    fn mul_assign(&mut self, rhs: i64) { self.0.mul_assign(rhs) }
}

impl Div<BlockPosition> for BlockPosition {
    type Output = Self;
    fn div(self, rhs: BlockPosition) -> Self::Output {
        Self::new(self.x.div(rhs.x), self.y.div(rhs.y), self.z.div(rhs.z))
    }
}

impl DivAssign<BlockPosition> for BlockPosition {
    #[inline]
    fn div_assign(&mut self, rhs: BlockPosition) { self.0.div_assign(rhs.0) }
}

impl Div<i64> for BlockPosition {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output {
        Self::new(self.x.div(rhs), self.y.div(rhs), self.z.div(rhs))
    }
}

impl DivAssign<i64> for BlockPosition {
    #[inline]
    fn div_assign(&mut self, rhs: i64) { self.0.div_assign(rhs) }
}

impl From<Vec3> for BlockPosition {
    fn from(vec: Vec3) -> Self { Self::from(vec.as_i64vec3()) }
}

impl From<BlockPosition> for Vec3 {
    fn from(pos: BlockPosition) -> Self { pos.as_vec3() }
}

impl From<DVec3> for BlockPosition {
    fn from(vec: DVec3) -> Self { Self::from(vec.as_i64vec3()) }
}

impl From<BlockPosition> for DVec3 {
    fn from(pos: BlockPosition) -> Self { pos.as_dvec3() }
}

impl From<IVec3> for BlockPosition {
    fn from(vec: IVec3) -> Self { Self::from(vec.as_i64vec3()) }
}

impl From<BlockPosition> for IVec3 {
    fn from(pos: BlockPosition) -> Self { pos.as_ivec3() }
}

impl<T: Into<i64>> From<[T; 3]> for BlockPosition {
    fn from([first, second, third]: [T; 3]) -> Self {
        Self::new(first.into(), second.into(), third.into())
    }
}

impl<T: Into<i64>> From<(T, T, T)> for BlockPosition {
    fn from((first, second, third): (T, T, T)) -> Self {
        Self::new(first.into(), second.into(), third.into())
    }
}
