// use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub,
// SubAssign};

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use glam::U8Vec2;

use super::BlockPosition;
use crate::section::Section;

/// A block's position inside of a [`Chunk`](crate::chunk::Chunk).
///
/// Automatically wraps around when going out of bounds.
///
/// # Note
///
/// `Y = 0` is always considered the bottom of the chunk,
/// regardless of the chunk's minimum height.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq, Hash))]
pub struct RelativePosition(U8Vec2, u16);

impl RelativePosition {
    /// Create a new [`RelativePosition`] from the given coordinates.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::RelativePosition;
    ///
    /// let block = RelativePosition::new(0, 0, 0);
    /// assert_eq!(block.x(), 0);
    /// assert_eq!(block.y(), 0);
    /// assert_eq!(block.z(), 0);
    ///
    /// let block = RelativePosition::new(1, 1, 1);
    /// assert_eq!(block.x(), 1);
    /// assert_eq!(block.y(), 1);
    /// assert_eq!(block.z(), 1);
    ///
    /// let block = RelativePosition::new(16, 16, 16);
    /// assert_eq!(block.x(), 0);
    /// assert_eq!(block.y(), 16);
    /// assert_eq!(block.z(), 0);
    /// ```
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "This will never happen, as the values are below 255"
    )]
    pub const fn new(x: u8, y: u16, z: u8) -> Self {
        Self(U8Vec2::new(x % Section::SIDE_LENGTH as u8, z % Section::SIDE_LENGTH as u8), y)
    }

    /// Create a new [`RelativePosition`] from the given absolute coordinates.
    ///
    /// This means the chunk offset must already be taken into account!
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::RelativePosition;
    ///
    /// let relative = RelativePosition::from_coordinates(0, 0, 0);
    /// assert_eq!(relative.x(), 0);
    /// assert_eq!(relative.y(), 0);
    /// assert_eq!(relative.z(), 0);
    ///
    /// let relative = RelativePosition::from_coordinates(16, 16, 0);
    /// assert_eq!(relative.x(), 0); // Wrapped around back to 0
    /// assert_eq!(relative.y(), 16); // Y does not wrap around
    /// assert_eq!(relative.z(), 0);
    ///
    /// let relative = RelativePosition::from_coordinates(17, 128, -1);
    /// assert_eq!(relative.x(), 1); // Wrapped around back to 1
    /// assert_eq!(relative.y(), 128); // Y does not wrap around
    /// assert_eq!(relative.z(), 15); // Wrapped around back to 15
    /// ```
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        reason = "These will never happen, as the values are below 255"
    )]
    pub const fn from_coordinates(x: i32, y: u16, z: i32) -> Self {
        Self::new(
            x.rem_euclid(Section::SIDE_LENGTH as i32) as u8,
            y,
            z.rem_euclid(Section::SIDE_LENGTH as i32) as u8,
        )
    }

    /// Create a new [`RelativePosition`] from the given [`BlockPosition`] and
    /// offset.
    ///
    /// # Panics
    ///
    /// Panics if the `BlockPosition` `Y` is less the offset
    /// or higher than `u16::MAX - offset`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_world::position::{BlockPosition, RelativePosition};
    ///
    /// let relative = RelativePosition::from_block(BlockPosition::new(0, 0, 0), 0);
    /// assert_eq!(relative.x(), 0);
    /// assert_eq!(relative.y(), 0);
    /// assert_eq!(relative.z(), 0);
    ///
    /// let relative = RelativePosition::from_block(BlockPosition::new(16, 16, 0), 0);
    /// assert_eq!(relative.x(), 0); // Wrapped around back to 0
    /// assert_eq!(relative.y(), 16); // Y is the same with no offset
    /// assert_eq!(relative.z(), 0);
    ///
    /// let relative = RelativePosition::from_block(BlockPosition::new(16, 16, 0), -16);
    /// assert_eq!(relative.x(), 0); // Wrapped around back to 0
    /// assert_eq!(relative.y(), 32); // Y is offset by 16
    /// assert_eq!(relative.z(), 0);
    /// ```
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "Truncation only happens at really high offsets, sign loss will never happen"
    )]
    pub const fn from_block(pos: BlockPosition, offset: isize) -> Self {
        const MIN: i32 = u16::MIN as i32;
        const MAX: i32 = u16::MAX as i32;

        match pos.y().checked_sub(offset as i32).expect("`BlockPosition` Y overflowed from offset!")
        {
            // Safe, inside of valid range
            pos_y @ MIN..=MAX => Self::from_coordinates(pos.x(), pos_y as u16, pos.z()),
            // Panic, outside of valid range
            _ => panic!("`BlockPosition` Y out of valid range!"),
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

    /// Returns the index of this position in a
    /// [`Section`](crate::section::Section)'.
    #[must_use]
    pub const fn as_section_index(self) -> usize {
        let (x, y, z) = (
            self.x() as usize % Section::SIDE_LENGTH,
            self.y() as usize % Section::SIDE_LENGTH,
            self.z() as usize % Section::SIDE_LENGTH,
        );

        x + (z * Section::SIDE_LENGTH) + (y * Section::SIDE_LENGTH * Section::SIDE_LENGTH)
    }
}
