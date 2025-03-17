use std::ops::{Add, AddAssign, Sub, SubAssign};

#[cfg(feature = "io")]
use froglight_io::prelude::*;
use glam::U8Vec2;

use super::BlockPos;
use crate::chunk::Section;

/// A relative position in a [`Chunk`](crate::chunk::Chunk).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

    /// Create a new [`RelativeBlockPos`] from the given [`BlockPos`] and
    /// offset.
    ///
    /// # Panics
    /// Panics if the `BlockPos` is lower vertically than the offset.
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
    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    pub const fn from_block(pos: BlockPos, offset: isize) -> Self {
        Self::new(
            pos.x().rem_euclid(Section::WIDTH as i32) as u8,
            pos.y().checked_add(offset as i32).expect("`BlockPos` out of range!") as u16,
            pos.z().rem_euclid(Section::DEPTH as i32) as u8,
        )
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
