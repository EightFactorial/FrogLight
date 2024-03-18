use std::{
    num::TryFromIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use bevy_math::{I64Vec2, IVec2};
use derive_more::{Deref, DerefMut, From, Into};

use crate::io::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite};

/// A position in the world, measured in chunks.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, From, Into, Deref, DerefMut)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ChunkPosition(#[cfg_attr(feature = "reflect", reflect(ignore))] I64Vec2);

impl ChunkPosition {
    /// All zeros.
    pub const ZERO: Self = Self(I64Vec2::ZERO);

    /// All ones.
    pub const ONE: Self = Self(I64Vec2::ONE);

    /// All `u64::MIN`.
    pub const MIN: Self = Self(I64Vec2::MIN);

    /// All `u64::MAX`.
    pub const MAX: Self = Self(I64Vec2::MAX);

    /// Creates a new [`ChunkPosition`] with the given coordinates.
    #[must_use]
    #[inline]
    pub const fn new(x: i64, z: i64) -> Self { Self(I64Vec2::new(x, z)) }

    /// Creates a new [`ChunkPosition`] where all coordinates are the same.
    #[must_use]
    #[inline]
    pub const fn splat(v: i64) -> Self { Self(I64Vec2::splat(v)) }

    /// Gets the x-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn x(&self) -> i64 { self.0.x }
    /// Gets the z-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn z(&self) -> i64 { self.0.y }
}

/// Read as i32s and then converted to i64s.
impl FrogRead for ChunkPosition {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        // Swap the X/Z coordinates
        let pos = IVec2::fg_read(buf)?;
        Ok(Self::new(i64::from(pos.y), i64::from(pos.x)))
    }
}

/// Read as variable i32s and then converted to i64s.
impl FrogVarRead for ChunkPosition {
    fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        // Swap the X/Z coordinates
        let pos = IVec2::fg_var_read(buf)?;
        Ok(Self::new(i64::from(pos.y), i64::from(pos.x)))
    }
}

/// Converted to i32s and then written.
impl FrogWrite for ChunkPosition {
    fn fg_write(
        &self,
        buf: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), crate::io::WriteError> {
        // Swap the X/Z coordinates
        IVec2::new(i32::try_from(self.y)?, i32::try_from(self.x)?).fg_write(buf)
    }
}

/// Converted to i32s and then variably written.
impl FrogVarWrite for ChunkPosition {
    fn fg_var_write(
        &self,
        buf: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), crate::io::WriteError> {
        // Swap the X/Z coordinates
        IVec2::new(i32::try_from(self.y)?, i32::try_from(self.x)?).fg_var_write(buf)
    }
}

// --- Math Implementations ---

impl Add<ChunkPosition> for ChunkPosition {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) }
}
impl AddAssign<ChunkPosition> for ChunkPosition {
    fn add_assign(&mut self, rhs: Self) { self.0.add_assign(rhs.0); }
}

impl Sub<ChunkPosition> for ChunkPosition {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) }
}
impl SubAssign<ChunkPosition> for ChunkPosition {
    fn sub_assign(&mut self, rhs: Self) { self.0.sub_assign(rhs.0); }
}

impl Mul<ChunkPosition> for ChunkPosition {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output { Self(self.0 * rhs.0) }
}
impl MulAssign<ChunkPosition> for ChunkPosition {
    fn mul_assign(&mut self, rhs: Self) { self.0.mul_assign(rhs.0); }
}

impl Mul<i64> for ChunkPosition {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output { Self(self.0 * rhs) }
}
impl MulAssign<i64> for ChunkPosition {
    fn mul_assign(&mut self, rhs: i64) { self.0.mul_assign(rhs); }
}

impl Mul<i32> for ChunkPosition {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output { Self(self.0 * i64::from(rhs)) }
}
impl MulAssign<i32> for ChunkPosition {
    fn mul_assign(&mut self, rhs: i32) { self.0.mul_assign(i64::from(rhs)); }
}

impl Div<ChunkPosition> for ChunkPosition {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output { Self(self.0 / rhs.0) }
}
impl DivAssign<ChunkPosition> for ChunkPosition {
    fn div_assign(&mut self, rhs: Self) { self.0.div_assign(rhs.0); }
}

impl Div<i64> for ChunkPosition {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output { Self(self.0 / rhs) }
}
impl DivAssign<i64> for ChunkPosition {
    fn div_assign(&mut self, rhs: i64) { self.0.div_assign(rhs); }
}

impl Div<i32> for ChunkPosition {
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output { Self(self.0 / i64::from(rhs)) }
}
impl DivAssign<i32> for ChunkPosition {
    fn div_assign(&mut self, rhs: i32) { self.0.div_assign(i64::from(rhs)); }
}

// --- Conversion Implementations ---

// Create implementations on groups of types.
macro_rules! impl_from {
    ($($from:ty),* => $to:ty) => {
        $(
            impl From<[$from; 2]> for $to {
                fn from([x, z]: [$from; 2]) -> Self {
                    Self::new(Into::into(x), Into::into(z))
                }
            }
            impl From<($from, $from)> for $to {
                fn from((x, z): ($from, $from)) -> Self {
                    Self::new(Into::into(x), Into::into(z))
                }
            }
        )*
    };
    (try $($from:ty),* => $to:ty) => {
        $(
            impl TryFrom<[$from; 2]> for $to {
                type Error = TryFromIntError;
                fn try_from([x, z]: [$from; 2]) -> Result<Self, Self::Error> {
                    Ok(Self::new(TryFrom::try_from(x)?,TryFrom::try_from(z)?))
                }
            }
            impl TryFrom<($from, $from)> for $to {
                type Error = TryFromIntError;
                fn try_from((x, z): ($from, $from)) -> Result<Self, Self::Error> {
                    Ok(Self::new(TryFrom::try_from(x)?, TryFrom::try_from(z)?))
                }
            }
        )*
    };
}

impl_from!(i64, i32, i16, i8 => ChunkPosition);
impl_from!(try u128, i128, isize, usize, u64, u32, u16, u8 => ChunkPosition);

impl TryFrom<ChunkPosition> for IVec2 {
    type Error = TryFromIntError;
    fn try_from(pos: ChunkPosition) -> Result<Self, Self::Error> {
        Ok(IVec2::new(i32::try_from(pos.x)?, i32::try_from(pos.y)?))
    }
}
impl From<IVec2> for ChunkPosition {
    fn from(vec: IVec2) -> Self { Self::new(vec.x.into(), vec.y.into()) }
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
    fn chunk_add([x, z] in proptest::array::uniform2(MIN_TEST..MAX_TEST)) {
        assert_eq!(ChunkPosition::new(x, z) + ChunkPosition::new(1, 2), ChunkPosition::new(x + 1, z + 2));
    }

    #[test]
    fn chunk_sub([x, z] in proptest::array::uniform2(MIN_TEST..MAX_TEST)) {
        assert_eq!(ChunkPosition::new(x, z) - ChunkPosition::new(1, 2), ChunkPosition::new(x - 1, z - 2));
    }

    #[test]
    fn chunk_mul([x, z] in proptest::array::uniform2(MIN_TEST/128..MAX_TEST/128), scalar in -128i64..=128i64) {
        if scalar != 0 {
            assert_eq!(ChunkPosition::new(x, z) * scalar, ChunkPosition::new(x * scalar, z * scalar));
        }
    }

    #[test]
    fn chunk_div([x, z] in proptest::array::uniform2(MIN_TEST..MAX_TEST), scalar in -128i64..=128i64) {
        if scalar != 0 {
            assert_eq!(ChunkPosition::new(x, z) / scalar, ChunkPosition::new(x / scalar,  z / scalar));
        }
    }
}
