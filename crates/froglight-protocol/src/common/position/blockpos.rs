use std::{
    num::TryFromIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use bevy_math::{I64Vec3, IVec3};
use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogTest;

use crate::io::{FrogRead, FrogWrite};

/// A position in the world, measured in blocks.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, From, Into, Deref, DerefMut, FrogTest,
)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct BlockPosition(#[cfg_attr(feature = "reflect", reflect(ignore))] I64Vec3);

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

impl BlockPosition {
    // <3 Azalea
    const PACKED_X_LENGTH: u64 = 1 + 25;
    const PACKED_Z_LENGTH: u64 = Self::PACKED_X_LENGTH;
    const PACKED_Y_LENGTH: u64 = 64 - Self::PACKED_X_LENGTH - Self::PACKED_Z_LENGTH;
    const PACKED_X_MASK: u64 = (1 << Self::PACKED_X_LENGTH) - 1;
    const PACKED_Y_MASK: u64 = (1 << Self::PACKED_Y_LENGTH) - 1;
    const PACKED_Z_MASK: u64 = (1 << Self::PACKED_Z_LENGTH) - 1;
    const Z_OFFSET: u64 = Self::PACKED_Y_LENGTH;
    const X_OFFSET: u64 = Self::PACKED_Y_LENGTH + Self::PACKED_Z_LENGTH;
}

/// Read as i32s and then converted to i64s.
impl FrogRead for BlockPosition {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let val = i64::fg_read(buf)?;

        let x =
            val << (64 - Self::X_OFFSET - Self::PACKED_X_LENGTH) >> (64 - Self::PACKED_X_LENGTH);

        let y = val << (64 - Self::PACKED_Y_LENGTH) >> (64 - Self::PACKED_Y_LENGTH);

        let z =
            val << (64 - Self::Z_OFFSET - Self::PACKED_Z_LENGTH) >> (64 - Self::PACKED_Z_LENGTH);

        Ok(Self::new(x, y, z))
    }
}

/// Converted to i32s and then written.
impl FrogWrite for BlockPosition {
    #[allow(clippy::cast_sign_loss)]
    fn fg_write(
        &self,
        buf: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), crate::io::WriteError> {
        let mut val: u64 = 0;
        val |= ((self.x as u64) & Self::PACKED_X_MASK) << Self::X_OFFSET;
        val |= (self.y as u64) & Self::PACKED_Y_MASK;
        val |= ((self.z as u64) & Self::PACKED_Z_MASK) << Self::Z_OFFSET;
        val.fg_write(buf)
    }
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
