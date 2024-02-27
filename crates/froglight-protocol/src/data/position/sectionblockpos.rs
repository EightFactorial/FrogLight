use std::{
    num::TryFromIntError,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use bevy_reflect::Reflect;

use super::{BlockPosition, ChunkBlockPosition};

/// A position in a section, with x, y, and z coordinates.
///
/// The range of each coordinate is `0..16`, not including `16`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct SectionBlockPosition {
    /// The x-coordinate of the position.
    pub x: u8,
    /// The y-coordinate of the position.
    pub y: u8,
    /// The z-coordinate of the position.
    pub z: u8,
}

impl SectionBlockPosition {
    /// Creates a new [`SectionBlockPosition`] with the given coordinates.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_protocol::data::SectionBlockPosition;
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
    /// use froglight_protocol::data::SectionBlockPosition;
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
    /// use froglight_protocol::data::SectionBlockPosition;
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
    /// use froglight_protocol::data::SectionBlockPosition;
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
    /// let pos = SectionBlockPosition::from_index(256);
    /// assert_eq!(pos, SectionBlockPosition::new(0, 1, 0));
    /// ```
    #[must_use]
    #[inline]
    #[allow(clippy::missing_panics_doc)]
    pub fn from_index(index: usize) -> Self {
        let x = u8::try_from(index.rem_euclid(16)).expect("Index is out of range");
        let z = u8::try_from(index.div_euclid(16).rem_euclid(16)).expect("Index is out of range");
        let y = u8::try_from(index.div_euclid(256).rem_euclid(16)).expect("Index is out of range");
        Self::new(x, y, z)
    }

    /// Adds a relative position to the position.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_protocol::data::{BlockPosition, SectionBlockPosition};
    ///
    /// // The block position inside the chunk
    /// let mut pos = SectionBlockPosition::new(1, 2, 3);
    ///
    /// // A relative position
    /// let rel = BlockPosition::new(1, 0, 0);
    ///
    /// // Move one block in the x-direction
    /// pos.add_relative(&rel);
    /// assert_eq!(pos, SectionBlockPosition::new(2, 2, 3));
    ///
    /// // Move another block in the x-direction
    /// pos.add_relative(&rel);
    /// assert_eq!(pos, SectionBlockPosition::new(3, 2, 3));
    /// ```
    #[inline]
    #[allow(clippy::missing_panics_doc)]
    pub fn add_relative(&mut self, pos: &BlockPosition) {
        let x = u8::try_from((i64::from(self.x) + pos.x()).rem_euclid(16))
            .expect("Index is out of range");
        let y = u8::try_from((i64::from(self.y) + pos.y()).rem_euclid(16))
            .expect("Index is out of range");
        let z = u8::try_from((i64::from(self.z) + pos.z()).rem_euclid(16))
            .expect("Index is out of range");

        self.x = x;
        self.y = y;
        self.z = z;
    }
}

// --- Math Implementations ---

impl Add<SectionBlockPosition> for SectionBlockPosition {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut x = self.x.wrapping_add(rhs.x);
        let mut y = self.y.wrapping_add(rhs.y);
        let mut z = self.z.wrapping_add(rhs.z);

        // Add the carry to the next coordinate
        if x > 15 {
            z = z.wrapping_add(1);
        }
        if z > 15 {
            y = y.wrapping_add(1);
        }

        // Ensure the coordinates are in the range 0..16
        x %= 16;
        y %= 16;
        z %= 16;

        Self { x, y, z }
    }
}
impl AddAssign<SectionBlockPosition> for SectionBlockPosition {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl Sub<SectionBlockPosition> for SectionBlockPosition {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut x = self.x.wrapping_sub(rhs.x);
        let mut y = self.y.wrapping_sub(rhs.y);
        let mut z = self.z.wrapping_sub(rhs.z);

        // Add the carry to the next coordinate
        if x > 15 {
            z = z.wrapping_sub(1);
        }
        if z > 15 {
            y = y.wrapping_sub(1);
        }

        // Ensure the coordinates are in the range 0..16
        x %= 16;
        y %= 16;
        z %= 16;

        Self { x, y, z }
    }
}
impl SubAssign<SectionBlockPosition> for SectionBlockPosition {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
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

impl_from!(group u8 => SectionBlockPosition);
impl_from!(try_group usize, u128, u64, u32, u16, isize, i128, i64, i32, i16, i8 => SectionBlockPosition);

// --- Tests ---

#[cfg(test)]
proptest::proptest! {

    #[test]
    fn chunkblock_index(index in proptest::bits::u32::ANY) {
        let pos = SectionBlockPosition::from_index(index as usize);
        let expected = index.rem_euclid(4096);
        assert_eq!(pos.as_index(), expected as usize);
    }

    #[test]
    fn chunkblock_add(index in proptest::bits::u32::ANY) {
        let pos = SectionBlockPosition::from_index(index as usize);
        let pos_index = index.rem_euclid(4096) as usize;

        let offset = SectionBlockPosition::new(1, 2, 3);
        let offset_index = offset.as_index();

        assert_eq!((pos + offset).as_index(), (pos_index + offset_index).rem_euclid(4096));
    }

    #[test]
    fn chunkblock_sub(index in proptest::bits::u32::ANY) {
        let pos = SectionBlockPosition::from_index(index as usize);
        let pos_index = index.rem_euclid(4096) as usize;

        let offset = SectionBlockPosition::new(1, 2, 3);
        let offset_index = offset.as_index();

        assert_eq!((pos - offset).as_index(), pos_index.wrapping_sub(offset_index).rem_euclid(4096));
    }
}
