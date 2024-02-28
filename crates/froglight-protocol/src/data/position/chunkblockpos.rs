use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy_reflect::Reflect;

use super::BlockPosition;

/// A position in a chunk, with x, y, and z coordinates.
///
/// The range of the x and z coordinates are `0..16`, not including `16`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkBlockPosition {
    /// The x-coordinate of the position.
    ///
    /// The range of possible values is `0..16`, not including `16`.
    pub x: u8,
    /// The y-coordinate of the position.
    ///
    /// ---
    ///
    /// The y-coordinate is world-relative, so between worlds the y-coordinate
    /// will be different.
    ///
    /// For example:
    /// In the Overworld where the world starts at -64, `y = 0` is at -64.
    ///
    /// In the Nether where the world starts at 0, `y = 0` is at 0.
    pub y: usize,
    /// The z-coordinate of the position.
    ///
    /// The range possible values is `0..16`, not including `16`.
    pub z: u8,
}

impl ChunkBlockPosition {
    /// Creates a new [`ChunkBlockPosition`] with the given coordinates.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_protocol::data::ChunkBlockPosition;
    ///
    /// let pos = ChunkBlockPosition::new(1, 2, 3);
    /// assert_eq!(pos.x(), 1);
    /// assert_eq!(pos.y(), 2);
    /// assert_eq!(pos.z(), 3);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(x: u8, y: usize, z: u8) -> Self {
        debug_assert!(x < 16, "X-coordinate is out of range");
        debug_assert!(z < 16, "Z-coordinate is out of range");
        Self { x, y, z }
    }

    /// Creates a new [`ChunkBlockPosition`] where all coordinates are the
    /// same.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_protocol::data::ChunkBlockPosition;
    ///
    /// let pos = ChunkBlockPosition::splat(5);
    /// assert_eq!(pos.x(), 5);
    /// assert_eq!(pos.y(), 5);
    /// assert_eq!(pos.z(), 5);
    /// ```
    #[must_use]
    #[inline]
    pub const fn splat(v: u8) -> Self {
        debug_assert!(v < 16, "Coordinate is out of range");
        Self::new(v, v as usize, v)
    }

    /// Gets the x-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn x(&self) -> u8 { self.x }
    /// Gets the y-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn y(&self) -> usize { self.y }
    /// Gets the z-coordinate of the position.
    #[must_use]
    #[inline]
    pub const fn z(&self) -> u8 { self.z }

    /// Attempts to add the given offset to the chunk block position.
    ///
    /// Returns [`None`] if the height is above [`isize::MAX`] or if
    /// `height_offset + pos.y < 0`.
    ///
    /// The `height_offset` is relative to each world.
    ///
    /// The vanilla offsets are:
    /// - Overworld: `-64`
    /// - Nether: `0`
    /// - End: `0`
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn add_relative(
        &self,
        height_offset: isize,
        pos: &BlockPosition,
    ) -> Option<ChunkBlockPosition> {
        // Add the height offset to the y-coordinate.
        let Ok(coord) = isize::try_from(pos.y) else {
            // Return None if the height is above isize::MAX (9,223,372,036,854,775,807).
            return None;
        };
        let Ok(offset_coord) = usize::try_from(coord + height_offset) else {
            // If the height is below 0, return None.
            return None;
        };

        // Return the new position.
        Some(ChunkBlockPosition::new(
            u8::try_from((i64::from(self.x) + pos.x).rem_euclid(16)).expect("Index out of range"),
            self.y + offset_coord,
            u8::try_from((i64::from(self.z) + pos.z).rem_euclid(16)).expect("Index out of range"),
        ))
    }
}

// --- Math Implementations ---

impl Add<ChunkBlockPosition> for ChunkBlockPosition {
    type Output = ChunkBlockPosition;
    fn add(self, rhs: ChunkBlockPosition) -> Self::Output {
        ChunkBlockPosition::new(
            self.x.wrapping_add(rhs.x).rem_euclid(16),
            self.y + rhs.y,
            self.z.wrapping_add(rhs.z).rem_euclid(16),
        )
    }
}
impl AddAssign<ChunkBlockPosition> for ChunkBlockPosition {
    fn add_assign(&mut self, rhs: ChunkBlockPosition) { *self = *self + rhs; }
}
impl Sub<ChunkBlockPosition> for ChunkBlockPosition {
    type Output = ChunkBlockPosition;
    fn sub(self, rhs: ChunkBlockPosition) -> Self::Output {
        ChunkBlockPosition::new(
            self.x.wrapping_sub(rhs.x).rem_euclid(16),
            self.y - rhs.y,
            self.z.wrapping_sub(rhs.z).rem_euclid(16),
        )
    }
}
impl SubAssign<ChunkBlockPosition> for ChunkBlockPosition {
    fn sub_assign(&mut self, rhs: ChunkBlockPosition) { *self = *self - rhs; }
}

// --- Tests ---

#[cfg(test)]
const TEST_XZ_RANGE: std::ops::Range<u8> = 0u8..16u8;

#[cfg(test)]
const TEST_Y_RANGE: std::ops::Range<usize> = usize::MIN..usize::MAX;

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn chunkblockpos_add([x1, z1, x2, z2] in proptest::array::uniform4(TEST_XZ_RANGE), y1 in TEST_Y_RANGE) {
        let pos1 = ChunkBlockPosition::new(x1, y1, z1);
        let pos2 = ChunkBlockPosition::new(x2, 0, z2);
        let added = pos1 + pos2;

        // Make sure the result is within the expected range.
        assert!(added.x < 16);
        assert_eq!(added.y, y1);
        assert!(added.z < 16);

        // Make sure the result is as expected.
        assert_eq!(added.x, x1.wrapping_add(x2).rem_euclid(16));
        assert_eq!(added.z, z1.wrapping_add(z2).rem_euclid(16));
    }

    #[test]
    fn chunkblockpos_sub([x1, z1, x2, z2] in proptest::array::uniform4(TEST_XZ_RANGE), y1 in TEST_Y_RANGE) {
        let pos1 = ChunkBlockPosition::new(x1, y1, z1);
        let pos2 = ChunkBlockPosition::new(x2, 0, z2);

        let subbed = pos1 - pos2;

        // Make sure the result is within the expected range.
        assert!(subbed.x < 16);
        assert_eq!(subbed.y, y1);
        assert!(subbed.z < 16);

        // Make sure the result is as expected.
        assert_eq!(subbed.x, x1.wrapping_sub(x2).rem_euclid(16));
        assert_eq!(subbed.z, z1.wrapping_sub(z2).rem_euclid(16));
    }

}
