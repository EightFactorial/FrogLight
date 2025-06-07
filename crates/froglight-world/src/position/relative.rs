use core::ops::{Add, AddAssign, Sub, SubAssign};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "io")]
use froglight_io::prelude::*;
use glam::U8Vec2;

use super::BlockPos;
use crate::chunk::Section;

/// A relative position in a [`Chunk`](crate::chunk::Chunk).
///
/// Wraps around if the values are larger than a chunk.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(opaque))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, PartialEq, Hash))]
pub struct RelativeBlockPos(U8Vec2, u16);

impl RelativeBlockPos {
    /// Create a new [`RelativeBlockPos`] from the given coordinates.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::RelativeBlockPos;
    ///
    /// let block = RelativeBlockPos::new(0, 0, 0);
    /// assert_eq!(block.x(), 0);
    /// assert_eq!(block.y(), 0);
    /// assert_eq!(block.z(), 0);
    ///
    /// let block = RelativeBlockPos::new(1, 1, 1);
    /// assert_eq!(block.x(), 1);
    /// assert_eq!(block.y(), 1);
    /// assert_eq!(block.z(), 1);
    ///
    /// let block = RelativeBlockPos::new(16, 16, 16);
    /// assert_eq!(block.x(), 0);
    /// assert_eq!(block.y(), 16);
    /// assert_eq!(block.z(), 0);
    /// ```
    #[must_use]
    #[expect(clippy::cast_possible_truncation)]
    pub const fn new(x: u8, y: u16, z: u8) -> Self {
        Self(U8Vec2::new(x % Section::WIDTH as u8, z % Section::DEPTH as u8), y)
    }

    /// Create a new [`RelativeBlockPos`] from the given absolute coordinates.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::RelativeBlockPos;
    ///
    /// let relative = RelativeBlockPos::from_coordinates(0, 0, 0);
    /// assert_eq!(relative.x(), 0);
    /// assert_eq!(relative.y(), 0);
    /// assert_eq!(relative.z(), 0);
    ///
    /// let relative = RelativeBlockPos::from_coordinates(16, 16, 0);
    /// assert_eq!(relative.x(), 0); // Wrapped around back to 0
    /// assert_eq!(relative.y(), 16); // Y does not wrap around
    /// assert_eq!(relative.z(), 0);
    ///
    /// let relative = RelativeBlockPos::from_coordinates(17, 128, -1);
    /// assert_eq!(relative.x(), 1); // Wrapped around back to 1
    /// assert_eq!(relative.y(), 128); // Y does not wrap around
    /// assert_eq!(relative.z(), 15); // Wrapped around back to 15
    /// ```
    #[must_use]
    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub const fn from_coordinates(x: i32, y: u16, z: i32) -> Self {
        Self::new(
            x.rem_euclid(Section::WIDTH as i32) as u8,
            y,
            z.rem_euclid(Section::DEPTH as i32) as u8,
        )
    }

    /// Create a new [`RelativeBlockPos`] from the given [`BlockPos`] and
    /// offset.
    ///
    /// # Panics
    /// Panics if the `BlockPos` `Y` is less the offset
    /// or higher than `u16::MAX - offset`.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::{BlockPos, RelativeBlockPos};
    ///
    /// let relative = RelativeBlockPos::from_block(BlockPos::new(0, 0, 0), 0);
    /// assert_eq!(relative.x(), 0);
    /// assert_eq!(relative.y(), 0);
    /// assert_eq!(relative.z(), 0);
    ///
    /// let relative = RelativeBlockPos::from_block(BlockPos::new(16, 16, 0), 0);
    /// assert_eq!(relative.x(), 0); // Wrapped around back to 0
    /// assert_eq!(relative.y(), 16); // Y is the same with no offset
    /// assert_eq!(relative.z(), 0);
    ///
    /// let relative = RelativeBlockPos::from_block(BlockPos::new(16, 16, 0), -16);
    /// assert_eq!(relative.x(), 0); // Wrapped around back to 0
    /// assert_eq!(relative.y(), 0); // Y is offset by -16
    /// assert_eq!(relative.z(), 0);
    /// ```
    #[must_use]
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub const fn from_block(pos: BlockPos, offset: isize) -> Self {
        const MIN: i32 = u16::MIN as i32;
        const MAX: i32 = u16::MAX as i32;

        match pos.y().checked_add(offset as i32).expect("`BlockPos` Y overflowed from offset!") {
            // Safe, inside of valid range
            pos_y @ MIN..=MAX => Self::from_coordinates(pos.x(), pos_y as u16, pos.z()),
            // Panic, outside of valid range
            _ => panic!("`BlockPos` Y out of valid range!"),
        }
    }

    /// The x-coordinate of this block.
    #[must_use]
    pub const fn x(&self) -> u8 { self.0.x }

    /// The y-coordinate of this block.
    #[must_use]
    pub const fn y(&self) -> u16 { self.1 }

    /// The z-coordinate of this block.
    #[must_use]
    pub const fn z(&self) -> u8 { self.0.y }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl FrogRead for RelativeBlockPos {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        let packed = u8::frog_read(buffer)?;
        let height = u16::frog_read(buffer)?;
        Ok(Self(U8Vec2::new(packed >> 4, packed & 0xF), height))
    }
}

#[cfg(feature = "io")]
impl FrogWrite for RelativeBlockPos {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        let mut written = 0;
        written += ((self.x() & 0xF) << 4 | self.z() & 0xF).frog_write(buffer)?;
        written += self.y().frog_write(buffer)?;
        Ok(written)
    }

    #[inline]
    fn frog_len(&self) -> usize { u8::frog_len(&0u8) + u16::frog_len(&0u16) }
}

// -------------------------------------------------------------------------------------------------

impl Add<RelativeBlockPos> for RelativeBlockPos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x().wrapping_add(rhs.x()),
            self.y().wrapping_add(rhs.y()),
            self.z().wrapping_add(rhs.z()),
        )
    }
}
impl AddAssign<RelativeBlockPos> for RelativeBlockPos {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl Sub<RelativeBlockPos> for RelativeBlockPos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x().wrapping_sub(rhs.x()),
            self.y().wrapping_sub(rhs.y()),
            self.z().wrapping_sub(rhs.z()),
        )
    }
}
impl SubAssign<RelativeBlockPos> for RelativeBlockPos {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
proptest::proptest! {
    #[test]
    fn verify(x in proptest::num::u8::ANY, y in proptest::num::u16::ANY, z in proptest::num::u8::ANY) {
        let pos = RelativeBlockPos::new(x, y, z);
        assert_eq!(pos.x(), x % 16);
        assert_eq!(pos.y(), y);
        assert_eq!(pos.z(), z % 16);
    }

    #[test]
    fn addition(x1 in proptest::num::u8::ANY, y1 in proptest::num::u16::ANY, z1 in proptest::num::u8::ANY,
                       x2 in proptest::num::u8::ANY, y2 in proptest::num::u16::ANY, z2 in proptest::num::u8::ANY) {
        let result = RelativeBlockPos::new(x1, y1, z1) + RelativeBlockPos::new(x2, y2, z2);
        assert_eq!(result.x(), x1.wrapping_add(x2) % 16);
        assert_eq!(result.y(), y1.wrapping_add(y2));
        assert_eq!(result.z(), z1.wrapping_add(z2) % 16);
    }

    #[test]
    fn subtraction(x1 in proptest::num::u8::ANY, y1 in proptest::num::u16::ANY, z1 in proptest::num::u8::ANY,
                          x2 in proptest::num::u8::ANY, y2 in proptest::num::u16::ANY, z2 in proptest::num::u8::ANY) {
        let result = RelativeBlockPos::new(x1, y1, z1) - RelativeBlockPos::new(x2, y2, z2);
        assert_eq!(result.x(), x1.wrapping_sub(x2) % 16);
        assert_eq!(result.y(), y1.wrapping_sub(y2));
        assert_eq!(result.z(), z1.wrapping_sub(z2) % 16);
    }

    #[test]
    fn block_position(data in proptest::array::uniform3(proptest::num::i32::ANY)) {
        let pos = crate::prelude::BlockPos::new(data[0], data[1].clamp(u16::MIN as i32, u16::MAX as i32), data[2]);
        let relative = RelativeBlockPos::from_block(pos, 0);
        assert_eq!(relative.x() as i32, pos.x().rem_euclid(16));
        assert_eq!(relative.z() as i32, pos.z().rem_euclid(16));
    }

    #[test]
    fn coord_position(data in proptest::array::uniform3(proptest::num::i32::ANY)) {
        let pos = crate::prelude::BlockPos::new(data[0], data[1].clamp(u16::MIN as i32, u16::MAX as i32), data[2]);
        let relative = RelativeBlockPos::from_block(pos, 0);
        assert_eq!(relative.x() as i32, pos.x().rem_euclid(16));
        assert_eq!(relative.z() as i32, pos.z().rem_euclid(16));
    }
}
