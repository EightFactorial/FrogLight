use std::{
    num::TryFromIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use bevy_math::{I64Vec3, IVec3};
use bevy_reflect::Reflect;
use derive_more::{Deref, DerefMut};

/// A position in the world, measured in blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Deref, DerefMut)]
pub struct BlockPosition(#[reflect(ignore)] I64Vec3);

impl BlockPosition {
    /// All zeros.
    pub const ZERO: Self = Self(I64Vec3::ZERO);

    /// All ones.
    pub const ONE: Self = Self(I64Vec3::ONE);

    /// All `u64::MIN`.
    pub const MIN: Self = Self(I64Vec3::MIN);

    /// All `u64::MAX`.
    pub const MAX: Self = Self(I64Vec3::MAX);

    /// Creates a new [`BlockPosition`] with the given coordinates.
    #[must_use]
    #[inline]
    pub const fn new(x: i64, y: i64, z: i64) -> Self { Self(I64Vec3::new(x, y, z)) }

    /// Creates a new [`BlockPosition`] where all coordinates are the same.
    #[must_use]
    #[inline]
    pub const fn splat(v: i64) -> Self { Self(I64Vec3::splat(v)) }

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

// --- Math Implementations ---

impl Add<BlockPosition> for BlockPosition {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) }
}
impl AddAssign<BlockPosition> for BlockPosition {
    fn add_assign(&mut self, rhs: Self) { self.0.add_assign(rhs.0); }
}

impl Sub<BlockPosition> for BlockPosition {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) }
}
impl SubAssign<BlockPosition> for BlockPosition {
    fn sub_assign(&mut self, rhs: Self) { self.0.sub_assign(rhs.0); }
}

impl Mul<BlockPosition> for BlockPosition {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output { Self(self.0 * rhs.0) }
}
impl MulAssign<BlockPosition> for BlockPosition {
    fn mul_assign(&mut self, rhs: Self) { self.0.mul_assign(rhs.0); }
}

impl Mul<i64> for BlockPosition {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output { Self::new(self.x * rhs, self.y * rhs, self.z * rhs) }
}
impl MulAssign<i64> for BlockPosition {
    fn mul_assign(&mut self, rhs: i64) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
        self.z.sub_assign(rhs);
    }
}

impl Mul<i32> for BlockPosition {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * i64::from(rhs), self.y * i64::from(rhs), self.z * i64::from(rhs))
    }
}
impl MulAssign<i32> for BlockPosition {
    fn mul_assign(&mut self, rhs: i32) {
        self.x.sub_assign(i64::from(rhs));
        self.y.sub_assign(i64::from(rhs));
        self.z.sub_assign(i64::from(rhs));
    }
}

impl Div<BlockPosition> for BlockPosition {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}
impl DivAssign<BlockPosition> for BlockPosition {
    fn div_assign(&mut self, rhs: Self) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
    }
}

impl Div<i64> for BlockPosition {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output { Self::new(self.x / rhs, self.y / rhs, self.z / rhs) }
}
impl DivAssign<i64> for BlockPosition {
    fn div_assign(&mut self, rhs: i64) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
        self.z.sub_assign(rhs);
    }
}

impl Div<i32> for BlockPosition {
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output {
        Self::new(self.x / i64::from(rhs), self.y / i64::from(rhs), self.z / i64::from(rhs))
    }
}
impl DivAssign<i32> for BlockPosition {
    fn div_assign(&mut self, rhs: i32) {
        self.x.sub_assign(i64::from(rhs));
        self.y.sub_assign(i64::from(rhs));
        self.z.sub_assign(i64::from(rhs));
    }
}

// --- Conversion Implementations ---

// Create implementations on groups of types.
macro_rules! impl_from {
    (group $($from:ty),* => $to:ty) => {
        $(
            impl From<[$from; 3]> for $to {
                fn from([x, y, z]: [$from; 3]) -> Self {
                    Self::new(Into::into(x), Into::into(y), Into::into(z))
                }
            }
            impl From<($from, $from, $from)> for $to {
                fn from((x, y, z): ($from, $from, $from)) -> Self {
                    Self::new(Into::into(x), Into::into(y), Into::into(z))
                }
            }
        )*
    };
    (try_group $($from:ty),* => $to:ty) => {
        $(
            impl TryFrom<[$from; 3]> for $to {
                type Error = TryFromIntError;
                fn try_from([x, y, z]: [$from; 3]) -> Result<Self, Self::Error> {
                    Ok(Self::new(TryFrom::try_from(x)?, TryFrom::try_from(y)?, TryFrom::try_from(z)?))
                }
            }
            impl TryFrom<($from, $from, $from)> for $to {
                type Error = TryFromIntError;
                fn try_from((x, y, z): ($from, $from, $from)) -> Result<Self, Self::Error> {
                    Ok(Self::new(TryFrom::try_from(x)?, TryFrom::try_from(y)?, TryFrom::try_from(z)?))
                }
            }
        )*
    };
}

impl_from!(group i64, i32, i16, i8 => BlockPosition);
impl_from!(try_group u128, i128, isize, usize, u64, u32, u16, u8 => BlockPosition);

impl From<I64Vec3> for BlockPosition {
    fn from(vec: I64Vec3) -> Self { Self(vec) }
}
impl From<BlockPosition> for I64Vec3 {
    fn from(pos: BlockPosition) -> Self { pos.0 }
}

impl TryFrom<BlockPosition> for IVec3 {
    type Error = TryFromIntError;
    fn try_from(pos: BlockPosition) -> Result<Self, Self::Error> {
        Ok(IVec3::new(
            TryFrom::try_from(pos.x)?,
            TryFrom::try_from(pos.y)?,
            TryFrom::try_from(pos.z)?,
        ))
    }
}
impl From<IVec3> for BlockPosition {
    fn from(vec: IVec3) -> Self {
        Self::new(Into::into(vec.x), Into::into(vec.y), Into::into(vec.z))
    }
}

// --- Tests ---

/// The minimum value for testing.
///
/// Prevents overflows when adding or subtracting.
#[cfg(test)]
const MIN_TEST: i64 = i64::MIN + 4096;

/// The maximum value for testing.
///
/// Prevents overflows when adding or subtracting.
#[cfg(test)]
const MAX_TEST: i64 = i64::MAX - 4096;

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn block_add([x, y, z] in proptest::array::uniform3(MIN_TEST..MAX_TEST)) {
        assert_eq!(BlockPosition::new(x, y, z) + BlockPosition::new(1, 2, 3), BlockPosition::new(x + 1, y + 2, z + 3));
    }

    #[test]
    fn block_sub([x, y, z] in proptest::array::uniform3(MIN_TEST..MAX_TEST)) {
        assert_eq!(BlockPosition::new(x, y, z) - BlockPosition::new(1, 2, 3), BlockPosition::new(x - 1, y - 2, z - 3));
    }

    #[test]
    fn block_mul([x, y, z] in proptest::array::uniform3(MIN_TEST/128..MAX_TEST/128), scalar in -128i64..=128i64) {
        if scalar != 0 {
            assert_eq!(BlockPosition::new(x, y, z) * scalar, BlockPosition::new(x * scalar, y * scalar, z * scalar));
        }
    }

    #[test]
    fn block_div([x, y, z] in proptest::array::uniform3(MIN_TEST..MAX_TEST), scalar in -128i64..=128i64) {
        if scalar != 0 {
            assert_eq!(BlockPosition::new(x, y, z) / scalar, BlockPosition::new(x / scalar, y / scalar, z / scalar));
        }
    }
}
