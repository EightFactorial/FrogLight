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

    /// Adds a relative position to the position.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_protocol::data::{BlockPosition, ChunkBlockPosition};
    ///
    /// // The block position inside the chunk
    /// let mut pos = ChunkBlockPosition::new(1, 2, 3);
    ///
    /// // A relative position
    /// let rel = BlockPosition::new(1, 0, 0);
    ///
    /// // Move one block in the x-direction
    /// pos.add_relative(&rel);
    /// assert_eq!(pos, ChunkBlockPosition::new(2, 2, 3));
    ///
    /// // Move another block in the x-direction
    /// pos.add_relative(&rel);
    /// assert_eq!(pos, ChunkBlockPosition::new(3, 2, 3));
    /// ```
    #[inline]
    #[allow(clippy::missing_panics_doc)]
    pub fn add_relative(&mut self, pos: &BlockPosition) {
        let x = u8::try_from((i64::from(self.x) + pos.x()).rem_euclid(16))
            .expect("Index is out of range");
        let y = self.y + usize::try_from(pos.y()).expect("Index is out of range");
        let z = u8::try_from((i64::from(self.z) + pos.z()).rem_euclid(16))
            .expect("Index is out of range");

        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl Add<ChunkBlockPosition> for ChunkBlockPosition {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut x = self.x.wrapping_add(rhs.x);
        let mut y = self.y + rhs.y;
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
        z %= 16;

        Self { x, y, z }
    }
}
impl AddAssign<ChunkBlockPosition> for ChunkBlockPosition {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl Sub<ChunkBlockPosition> for ChunkBlockPosition {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut x = self.x.wrapping_sub(rhs.x);
        let mut y = self.y - rhs.y;
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
        z %= 16;

        Self { x, y, z }
    }
}
impl SubAssign<ChunkBlockPosition> for ChunkBlockPosition {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}

// --- Tests ---

#[cfg(test)]
proptest::proptest! {

    #[test]
    fn chunkblock_add([x, z] in proptest::array::uniform2(0u8..16u8), y in proptest::bits::u32::ANY) {
        let pos = ChunkBlockPosition::new(x, y as usize, z);
        let rel = ChunkBlockPosition::new(1, 2, 3);
        let output = pos + rel;

        // Ensure the coordinates are in the range 0..16
        assert!(output.x < 16, "X-coordinate is out of range");
        assert!(output.z < 16, "Z-coordinate is out of range");

        // Ensure the x-coordinate is correct
        let expected = x.wrapping_add(1).rem_euclid(16);
        assert_eq!(output.x, expected, "X-coordinate is incorrect");

        // Ensure the y-coordinate is correct
        let mut expected = y as usize + 2;
        if (pos.x >= 15 && pos.z > 11) || pos.z > 12 {
            expected += 1;
        }
        assert_eq!(output.y, expected, "Y-coordinate is incorrect");

        // Ensure the z-coordinate is correct
        let mut expected = z.wrapping_add(3);
        if pos.x >= 15 {
            expected += 1;
        }
        assert_eq!(output.z, expected.rem_euclid(16), "Z-coordinate is incorrect");
    }

    #[test]
    fn chunkblock_sub([x, z] in proptest::array::uniform2(0u8..16u8), y in proptest::bits::u32::ANY) {
        // Ensure the y-coordinate is at least 2 to prevent underflow
        if y < 2 {
            return Ok(());
        }

        let pos = ChunkBlockPosition::new(x, y as usize, z);
        let rel = ChunkBlockPosition::new(1, 2, 3);
        let output = pos - rel;

        // Ensure the coordinates are in the range 0..16
        assert!(output.x < 16, "X-coordinate is out of range");
        assert!(output.z < 16, "Z-coordinate is out of range");

        // Ensure the x-coordinate is correct
        let expected = x.wrapping_sub(1).rem_euclid(16);
        assert_eq!(output.x, expected, "X-coordinate is incorrect");

        // Ensure the y-coordinate is correct
        let mut expected = y as usize - 2;
        if (pos.x < 1 && pos.z < 4 ) || pos.z < 3 {
            expected -= 1;
        }
        assert_eq!(output.y, expected, "Y-coordinate is incorrect");

        // Ensure the z-coordinate is correct
        let mut expected = z.wrapping_sub(3);
        if pos.x < 1 {
            expected = expected.wrapping_sub(1);
        }
        assert_eq!(output.z, expected.rem_euclid(16), "Z-coordinate is incorrect");
    }
}
