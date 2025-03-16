use std::ops::{Add, AddAssign, Sub, SubAssign};

use glam::IVec3;

use super::BlockPos;
use crate::chunk::Section;

/// A block position in a [`Section`](crate::prelude::Section).
///
/// Wraps around if the values are greater than the section's width.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_io::prelude::FrogBuf))]
pub struct SectionBlockPos(u16);

impl SectionBlockPos {
    pub const BITS: u8 = Self::MASK.leading_zeros() as u8;
    pub const MASK: u8 = Section::WIDTH as u8 - 1;

    /// Create a new [`SectionBlockPos`] from the given coordinates.
    ///
    /// The values will wrap around if they are greater than a section's width.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::SectionBlockPos;
    ///
    /// let block = SectionBlockPos::new(0, 0, 0);
    /// assert_eq!(block.x(), 0);
    /// assert_eq!(block.y(), 0);
    /// assert_eq!(block.z(), 0);
    ///
    /// let block = SectionBlockPos::new(1, 1, 1);
    /// assert_eq!(block.x(), 1);
    /// assert_eq!(block.y(), 1);
    /// assert_eq!(block.z(), 1);
    ///
    /// let block = SectionBlockPos::new(15, 15, 15);
    /// assert_eq!(block.x(), 15);
    /// assert_eq!(block.y(), 15);
    /// assert_eq!(block.z(), 15);
    ///
    /// // Values greater than 15 will wrap around.
    /// let block = SectionBlockPos::new(16, 16, 16);
    /// assert_eq!(block.x(), 0);
    /// assert_eq!(block.y(), 0);
    /// assert_eq!(block.z(), 0);
    /// ```
    #[must_use]
    pub const fn new(x: u8, y: u8, z: u8) -> Self {
        Self(
            (x & Self::MASK) as u16
                | ((y & Self::MASK) as u16) << Self::BITS
                | ((z & Self::MASK) as u16) << (Self::BITS * 2),
        )
    }

    /// The x-coordinate of this block.
    #[must_use]
    pub const fn x(&self) -> u8 { self.0 as u8 & Self::MASK }

    /// The y-coordinate of this block.
    #[must_use]
    pub const fn y(&self) -> u8 { (self.0 >> Self::BITS) as u8 & Self::MASK }

    /// The z-coordinate of this block.
    #[must_use]
    pub const fn z(&self) -> u8 { (self.0 >> (Self::BITS * 2)) as u8 & Self::MASK }

    /// Create a [`SectionBlockPos`] from the given [`BlockPos`].
    ///
    /// The values will wrap around if they are greater than a section's width.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::{BlockPos, SectionBlockPos};
    ///
    /// let block = BlockPos::new(0, 0, 0);
    /// assert_eq!(SectionBlockPos::from_block(block), SectionBlockPos::new(0, 0, 0));
    ///
    /// let block = BlockPos::new(1, 0, 1);
    /// assert_eq!(SectionBlockPos::from_block(block), SectionBlockPos::new(1, 0, 1));
    ///
    /// let block = BlockPos::new(16, 0, 16);
    /// assert_eq!(SectionBlockPos::from_block(block), SectionBlockPos::new(0, 0, 0));
    ///
    /// let block = BlockPos::new(-1, 0, 0);
    /// assert_eq!(SectionBlockPos::from_block(block), SectionBlockPos::new(15, 0, 0));
    ///
    /// let block = BlockPos::new(-16, 15, 16);
    /// assert_eq!(SectionBlockPos::from_block(block), SectionBlockPos::new(0, 15, 0));
    ///
    /// let block = BlockPos::new(0, 16, 0);
    /// assert_eq!(SectionBlockPos::from_block(block), SectionBlockPos::new(0, 0, 0));
    ///
    /// let block = BlockPos::new(1024, 0, -1000);
    /// assert_eq!(SectionBlockPos::from(block), SectionBlockPos::new(0, 0, 8));
    /// ```
    #[must_use]
    pub const fn from_block(block: BlockPos) -> Self {
        Self::new(block.x() as u8, block.y() as u8, block.z() as u8)
    }
}

// -------------------------------------------------------------------------------------------------

impl Into<IVec3> for SectionBlockPos {
    fn into(self) -> IVec3 { IVec3::new(self.x() as i32, self.y() as i32, self.z() as i32) }
}

impl From<BlockPos> for SectionBlockPos {
    fn from(value: BlockPos) -> Self { Self::from_block(value) }
}

// -------------------------------------------------------------------------------------------------

impl Add<SectionBlockPos> for SectionBlockPos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x().wrapping_add(rhs.x()),
            self.y().wrapping_add(rhs.y()),
            self.z().wrapping_add(rhs.z()),
        )
    }
}
impl AddAssign<SectionBlockPos> for SectionBlockPos {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl Sub<SectionBlockPos> for SectionBlockPos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x().wrapping_sub(rhs.x()),
            self.y().wrapping_sub(rhs.y()),
            self.z().wrapping_sub(rhs.z()),
        )
    }
}
impl SubAssign<SectionBlockPos> for SectionBlockPos {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}
