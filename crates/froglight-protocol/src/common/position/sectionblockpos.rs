use std::{
    num::TryFromIntError,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use super::{BlockPosition, ChunkBlockPosition};

/// A position in a section, with x, y, and z coordinates.
///
/// The range of each coordinate is `0..16`, not including `16`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct SectionBlockPosition {
    /// The x-coordinate of the position.
    pub x: u8,
    /// The y-coordinate of the position.
    pub y: u8,
    /// The z-coordinate of the position.
    pub z: u8,
}

impl SectionBlockPosition {
    /// All zeros.
    pub const ZERO: Self = Self::splat(0);

    /// The minimum position.
    ///
    /// This is the same as [`SectionBlockPosition::ZERO`].
    pub const MIN: Self = Self::ZERO;

    /// All `15`s.
    pub const FIFTEEN: Self = Self::splat(15);

    /// The maximum position.
    ///
    /// This is the same as [`SectionBlockPosition::FIFTEEN`].
    pub const MAX: Self = Self::FIFTEEN;

    /// Creates a new [`SectionBlockPosition`] with the given coordinates.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_protocol::common::SectionBlockPosition;
    ///
    /// let pos = SectionBlockPosition::new(1, 2, 3);
    /// assert_eq!(pos.x(), 1);
    /// assert_eq!(pos.y(), 2);
    /// assert_eq!(pos.z(), 3);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(x: u8, y: u8, z: u8) -> Self {
        debug_assert!(x < 16, "X-coordinate is out of range");
        debug_assert!(y < 16, "Y-coordinate is out of range");
        debug_assert!(z < 16, "Z-coordinate is out of range");
        Self { x, y, z }
    }

    /// Creates a new [`SectionBlockPosition`] where all coordinates are the
    /// same.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_protocol::common::SectionBlockPosition;
    ///
    /// let pos = SectionBlockPosition::splat(5);
    /// assert_eq!(pos.x(), 5);
    /// assert_eq!(pos.y(), 5);
    /// assert_eq!(pos.z(), 5);
    /// ```
    #[must_use]
    #[inline]
    pub const fn splat(v: u8) -> Self {
        debug_assert!(v < 16, "Coordinate is out of range");
        Self::new(v, v, v)
    }

    /// Gets the x-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn x(&self) -> u8 { self.x }
    /// Gets the y-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn y(&self) -> u8 { self.y }
    /// Gets the z-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn z(&self) -> u8 { self.z }

    /// Gets the index of the position in a flat array.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_protocol::common::SectionBlockPosition;
    ///
    /// // (1 * 1) + (0 * 256) + (0 * 16) = 1
    /// let pos = SectionBlockPosition::new(1, 0, 0);
    /// assert_eq!(pos.as_index(), 1);
    ///
    /// // (0 * 1) + (1 * 256) + (0 * 16) = 256
    /// let pos = SectionBlockPosition::new(0, 1, 0);
    /// assert_eq!(pos.as_index(), 256);
    ///
    /// // (0 * 1) + (0 * 256) + (1 * 16) = 16
    /// let pos = SectionBlockPosition::new(0, 0, 1);
    /// assert_eq!(pos.as_index(), 16);
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_index(&self) -> usize {
        (self.x as usize) + (self.z as usize * 16) + (self.y as usize * 256)
    }

    /// Creates a new [`SectionBlockPosition`] from an index in a flat array.
    ///
    /// Example:
    /// ```rust
    /// use froglight_protocol::common::SectionBlockPosition;
    ///
    /// let pos = SectionBlockPosition::from_index(0);
    /// assert_eq!(pos, SectionBlockPosition::new(0, 0, 0));
    ///
    /// let pos = SectionBlockPosition::from_index(1);
    /// assert_eq!(pos, SectionBlockPosition::new(1, 0, 0));
    ///
    /// let pos = SectionBlockPosition::from_index(16);
    /// assert_eq!(pos, SectionBlockPosition::new(0, 0, 1));
    ///
    /// let pos = SectionBlockPosition::from_index(17);
    /// assert_eq!(pos, SectionBlockPosition::new(1, 0, 1));
    ///
    /// let pos = SectionBlockPosition::from_index(256);
    /// assert_eq!(pos, SectionBlockPosition::new(0, 1, 0));
    ///
    /// let pos = SectionBlockPosition::from_index(4095);
    /// assert_eq!(pos, SectionBlockPosition::new(15, 15, 15));
    /// ```
    ///
    /// # Panics
    /// Panics if the index is greater than or equal to `4096`.
    #[must_use]
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_index(index: usize) -> Self {
        debug_assert!(index < 4096, "Index is out of range");

        let x = index.rem_euclid(16) as u8;
        let z = index.div_euclid(16).rem_euclid(16) as u8;
        let y = index.div_euclid(256).rem_euclid(16) as u8;

        Self::new(x, y, z)
    }
}

// --- Math Implementations ---

impl Add<SectionBlockPosition> for SectionBlockPosition {
    type Output = Self;
    fn add(self, rhs: SectionBlockPosition) -> Self::Output {
        Self::new(
            self.x.wrapping_add(rhs.x).rem_euclid(16),
            self.y.wrapping_add(rhs.y).rem_euclid(16),
            self.z.wrapping_add(rhs.z).rem_euclid(16),
        )
    }
}
impl AddAssign<SectionBlockPosition> for SectionBlockPosition {
    fn add_assign(&mut self, rhs: SectionBlockPosition) { *self = *self + rhs; }
}
impl Sub<SectionBlockPosition> for SectionBlockPosition {
    type Output = Self;
    fn sub(self, rhs: SectionBlockPosition) -> Self::Output {
        Self::new(
            self.x.wrapping_sub(rhs.x).rem_euclid(16),
            self.y.wrapping_sub(rhs.y).rem_euclid(16),
            self.z.wrapping_sub(rhs.z).rem_euclid(16),
        )
    }
}
impl SubAssign<SectionBlockPosition> for SectionBlockPosition {
    fn sub_assign(&mut self, rhs: SectionBlockPosition) { *self = *self - rhs; }
}

impl Add<BlockPosition> for SectionBlockPosition {
    type Output = SectionBlockPosition;
    fn add(self, rhs: BlockPosition) -> Self::Output {
        Self::new(
            u8::try_from((i64::from(self.x) + rhs.x).rem_euclid(16)).expect("Index out of range"),
            u8::try_from((i64::from(self.y) + rhs.y).rem_euclid(16)).expect("Index out of range"),
            u8::try_from((i64::from(self.z) + rhs.z).rem_euclid(16)).expect("Index out of range"),
        )
    }
}
impl AddAssign<BlockPosition> for SectionBlockPosition {
    fn add_assign(&mut self, rhs: BlockPosition) { *self = *self + rhs; }
}
impl Sub<BlockPosition> for SectionBlockPosition {
    type Output = SectionBlockPosition;
    fn sub(self, rhs: BlockPosition) -> Self::Output {
        Self::new(
            u8::try_from((i64::from(self.x) - rhs.x).rem_euclid(16)).expect("Index out of range"),
            u8::try_from((i64::from(self.y) - rhs.y).rem_euclid(16)).expect("Index out of range"),
            u8::try_from((i64::from(self.z) - rhs.z).rem_euclid(16)).expect("Index out of range"),
        )
    }
}
impl SubAssign<BlockPosition> for SectionBlockPosition {
    fn sub_assign(&mut self, rhs: BlockPosition) { *self = *self - rhs; }
}

impl From<ChunkBlockPosition> for SectionBlockPosition {
    fn from(pos: ChunkBlockPosition) -> Self {
        Self::new(pos.x, u8::try_from(pos.y.rem_euclid(16)).expect("Index out of range"), pos.z)
    }
}
impl From<&ChunkBlockPosition> for SectionBlockPosition {
    fn from(pos: &ChunkBlockPosition) -> Self {
        Self::new(pos.x, u8::try_from(pos.y.rem_euclid(16)).expect("Index out of range"), pos.z)
    }
}

// --- Conversion Implementations ---

// Create implementations on groups of types.
macro_rules! impl_from {
    ($($from:ty),* => $to:ty) => {
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
    (try $($from:ty),* => $to:ty) => {
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

impl_from!(u8 => SectionBlockPosition);
impl_from!(try usize, u128, u64, u32, u16, isize, i128, i64, i32, i16, i8 => SectionBlockPosition);

// --- Tests ---

#[cfg(test)]
const TEST_XYZ_RANGE: std::ops::Range<u8> = 0u8..16u8;

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn sectionblockpos_add([x1, y1, z1, x2, y2, z2] in proptest::array::uniform6(TEST_XYZ_RANGE)) {
        let pos1 = SectionBlockPosition::new(x1, y1, z1);
        let pos2 = SectionBlockPosition::new(x2, y2, z2);
        let added = pos1 + pos2;

        // Make sure the result is within the expected range.
        assert!(added.x < 16);
        assert!(added.y < 16);
        assert!(added.z < 16);

        // Make sure the result is as expected.
        assert_eq!(added.x, x1.wrapping_add(x2).rem_euclid(16));
        assert_eq!(added.y, y1.wrapping_add(y2).rem_euclid(16));
        assert_eq!(added.z, z1.wrapping_add(z2).rem_euclid(16));
    }

    #[test]
    fn sectionblockpos_sub([x1, y1, z1, x2, y2, z2] in proptest::array::uniform6(TEST_XYZ_RANGE))  {
        let pos1 = SectionBlockPosition::new(x1, y1, z1);
        let pos2 = SectionBlockPosition::new(x2, y2, z2);
        let subbed = pos1 - pos2;

        // Make sure the result is within the expected range.
        assert!(subbed.x < 16);
        assert!(subbed.y < 16);
        assert!(subbed.z < 16);

        // Make sure the result is as expected.
        assert_eq!(subbed.x, x1.wrapping_sub(x2).rem_euclid(16));
        assert_eq!(subbed.y, y1.wrapping_sub(y2).rem_euclid(16));
        assert_eq!(subbed.z, z1.wrapping_sub(z2).rem_euclid(16));
    }

}
