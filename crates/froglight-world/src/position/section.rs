use core::ops::{Add, AddAssign, Sub, SubAssign};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use glam::IVec3;

use super::BlockPos;
use crate::chunk::Section;

/// A block position in a [`Section`](crate::prelude::Section).
///
/// Wraps around if the values are larger than a section.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_io::prelude::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq, Hash))]
pub struct SectionBlockPos(u16);

impl SectionBlockPos {
    #[expect(clippy::cast_possible_truncation)]
    const BITS: u8 = (u8::BITS - Self::MASK.leading_zeros()) as u8;
    #[expect(clippy::cast_possible_truncation)]
    const MASK: u8 = Section::HEIGHT as u8 - 1;

    /// Create a new [`SectionBlockPos`] from the given coordinates.
    ///
    /// The values will wrap around if they are larger than a section.
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

    /// Create a new [`SectionBlockPos`] with all coordinates set to the same
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::SectionBlockPos;
    ///
    /// let block = SectionBlockPos::splat(0);
    /// assert_eq!(block.x(), 0);
    /// assert_eq!(block.y(), 0);
    /// assert_eq!(block.z(), 0);
    ///
    /// let block = SectionBlockPos::splat(1);
    /// assert_eq!(block.x(), 1);
    /// assert_eq!(block.y(), 1);
    /// assert_eq!(block.z(), 1);
    ///
    /// let block = SectionBlockPos::splat(255);
    /// assert_eq!(block.x(), 15);
    /// assert_eq!(block.y(), 15);
    /// assert_eq!(block.z(), 15);
    /// ```
    #[inline]
    #[must_use]
    pub const fn splat(v: u8) -> Self { Self::new(v, v, v) }

    /// The x-coordinate of this block.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn x(&self) -> u8 { self.0 as u8 & Self::MASK }

    /// The y-coordinate of this block.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn y(&self) -> u8 { (self.0 >> Self::BITS) as u8 & Self::MASK }

    /// The z-coordinate of this block.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn z(&self) -> u8 { (self.0 >> (Self::BITS * 2)) as u8 & Self::MASK }

    /// Create a [`SectionBlockPos`] from the given [`BlockPos`].
    ///
    /// The values will wrap around if they are larger than a section.
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
    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn from_block(block: BlockPos) -> Self {
        Self::new(
            block.x().rem_euclid(Section::WIDTH as i32) as _,
            block.y().rem_euclid(Section::HEIGHT as i32) as _,
            block.z().rem_euclid(Section::DEPTH as i32) as _,
        )
    }

    /// Create a [`SectionBlockPos`] from the given index.
    ///
    /// # Panics
    /// This function will panic if the index is greater than `4095`.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::SectionBlockPos;
    ///
    /// let block = SectionBlockPos::from_index(0);
    /// assert_eq!(block, SectionBlockPos::new(0, 0, 0));
    ///
    /// let block = SectionBlockPos::from_index(1);
    /// assert_eq!(block, SectionBlockPos::new(1, 0, 0));
    ///
    /// let block = SectionBlockPos::from_index(16);
    /// assert_eq!(block, SectionBlockPos::new(0, 0, 1));
    ///
    /// let block = SectionBlockPos::from_index(255);
    /// assert_eq!(block, SectionBlockPos::new(15, 0, 15));
    ///
    /// let block = SectionBlockPos::from_index(256);
    /// assert_eq!(block, SectionBlockPos::new(0, 1, 0));
    ///
    /// let block = SectionBlockPos::from_index(4095);
    /// assert_eq!(block, SectionBlockPos::new(15, 15, 15));
    /// ```
    #[must_use]
    #[expect(clippy::cast_possible_truncation)]
    pub const fn from_index(index: usize) -> Self {
        assert!(index < 4096, "Section index out of bounds!");

        let x = index.rem_euclid(Section::WIDTH) as u8;
        let z = index.div_euclid(Section::WIDTH).rem_euclid(Section::DEPTH) as u8;
        let y = index.div_euclid(Section::WIDTH * Section::DEPTH).rem_euclid(Section::HEIGHT) as u8;

        Self::new(x, y, z)
    }

    /// Get the index of this [`SectionBlockPos`].
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::SectionBlockPos;
    ///
    /// let block = SectionBlockPos::new(0, 0, 0);
    /// assert_eq!(block.into_index(), 0);
    ///
    /// let block = SectionBlockPos::new(1, 0, 0);
    /// assert_eq!(block.into_index(), 1);
    ///
    /// let block = SectionBlockPos::new(0, 0, 1);
    /// assert_eq!(block.into_index(), 16);
    ///
    /// let block = SectionBlockPos::new(0, 1, 0);
    /// assert_eq!(block.into_index(), 256);
    ///
    /// let block = SectionBlockPos::new(15, 15, 15);
    /// assert_eq!(block.into_index(), 4095);
    /// ```
    #[inline]
    #[must_use]
    pub const fn into_index(self) -> usize { self.into_quantized_index::<1>() }

    /// Get the index of this [`SectionBlockPos`] using the given scale.
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::position::SectionBlockPos;
    ///
    /// let block = SectionBlockPos::new(15, 15, 15);
    ///
    /// // 1:1 scale
    /// assert_eq!(block.into_quantized_index::<1>(), 4095);
    ///
    /// // 8:1 scale
    /// assert_eq!(block.into_quantized_index::<2>(), 511);
    ///
    /// // 16:1 scale
    /// assert_eq!(block.into_quantized_index::<4>(), 63);
    ///
    /// // 32:1 scale
    /// assert_eq!(block.into_quantized_index::<8>(), 7);
    ///
    /// // When `QUANTIZATION = 1` each position represents one block.
    /// //  _________________________
    /// // |\         \         \
    /// // | \         \         \
    /// // |  \_________\_________\_____
    /// // |  |\         \         \
    /// // |\ | \         \         \
    /// // | \|  \_________\_________\_____
    /// // |  \  |         |         |
    /// // \  |\ | (0,1,0) | (1,1,0) |
    /// //  \ | \|_________|_________|_____
    /// //   \|  |         |         |
    /// //    \  | (0,0,0) | (1,0,0) |
    /// //     \ |_________|_________|_____
    ///
    /// let block = SectionBlockPos::new(0, 0, 0);
    /// assert_eq!(block.into_quantized_index::<1>(), 0);
    ///
    /// let block = SectionBlockPos::new(1, 0, 0);
    /// assert_eq!(block.into_quantized_index::<1>(), 1);
    ///
    /// let block = SectionBlockPos::new(2, 0, 0);
    /// assert_eq!(block.into_quantized_index::<1>(), 2);
    ///
    /// let block = SectionBlockPos::new(3, 0, 0);
    /// assert_eq!(block.into_quantized_index::<1>(), 3);
    ///
    /// // When `QUANTIZATION = 2` each position represents 1/4 block.
    /// //  _________________________
    /// // |\                   \
    /// // | \                   \
    /// // |  \                   \
    /// // |   \                   \
    /// // |    \                   \
    /// // |     \___________________\_____
    /// // |     |                   |
    /// // \     | (0,1,0)   (1,1,0) |
    /// //  \    |                   |
    /// //   \   |                   |
    /// //    \  | (0,0,0)   (1,0,0) |
    /// //     \ |___________________|_____
    ///
    /// let block = SectionBlockPos::new(0, 0, 0);
    /// assert_eq!(block.into_quantized_index::<2>(), 0);
    ///
    /// let block = SectionBlockPos::new(1, 0, 0);
    /// assert_eq!(block.into_quantized_index::<2>(), 0);
    ///
    /// let block = SectionBlockPos::new(0, 0, 1);
    /// assert_eq!(block.into_quantized_index::<2>(), 0);
    ///
    /// let block = SectionBlockPos::new(0, 1, 0);
    /// assert_eq!(block.into_quantized_index::<2>(), 0);
    ///
    /// let block = SectionBlockPos::new(1, 1, 1);
    /// assert_eq!(block.into_quantized_index::<2>(), 0);
    /// ```
    #[must_use]
    pub const fn into_quantized_index<const QUANTIZATION: usize>(self) -> usize {
        let width = Section::WIDTH / QUANTIZATION;
        let depth = Section::DEPTH / QUANTIZATION;

        (self.x() as usize / QUANTIZATION)
            + (self.z() as usize / QUANTIZATION * width)
            + (self.y() as usize / QUANTIZATION * width * depth)
    }
}

// -------------------------------------------------------------------------------------------------

impl From<SectionBlockPos> for IVec3 {
    fn from(value: SectionBlockPos) -> Self {
        IVec3::new(value.x().into(), value.y().into(), value.z().into())
    }
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

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
proptest::proptest! {
    #[test]
    fn verify(data in proptest::array::uniform3(proptest::num::u8::ANY)) {
        let pos = SectionBlockPos::new(data[0], data[1], data[2]);
        assert_eq!(SectionBlockPos::from_index(pos.into_index()), pos);
        assert_eq!(pos.x(), data[0] % 16);
        assert_eq!(pos.y(), data[1] % 16);
        assert_eq!(pos.z(), data[2] % 16);
    }

    #[test]
    fn index(index in 0usize..4096usize) {
        let pos = SectionBlockPos::from_index(index);
        assert_eq!(pos.into_index(), index);
        assert!(pos.x() <= 16);
        assert!(pos.y() <= 16);
        assert!(pos.z() <= 16);
    }

    #[test]
    fn addition(data in proptest::array::uniform6(proptest::num::u8::ANY)) {
        let result = SectionBlockPos::new(data[0], data[1], data[2]) + SectionBlockPos::new(data[3], data[4], data[5]);
        assert_eq!(result.x(), data[0].wrapping_add(data[3]) % 16);
        assert_eq!(result.y(), data[1].wrapping_add(data[4]) % 16);
        assert_eq!(result.z(), data[2].wrapping_add(data[5]) % 16);
    }

    #[test]
    fn subtraction(data in proptest::array::uniform6(proptest::num::u8::ANY)) {
        let result = SectionBlockPos::new(data[0], data[1], data[2]) - SectionBlockPos::new(data[3], data[4], data[5]);
        assert_eq!(result.x(), data[0].wrapping_sub(data[3]) % 16);
        assert_eq!(result.y(), data[1].wrapping_sub(data[4]) % 16);
        assert_eq!(result.z(), data[2].wrapping_sub(data[5]) % 16);
    }

    #[test]
    fn block_position(data in proptest::array::uniform3(proptest::num::i32::ANY)) {
        let pos = SectionBlockPos::from_block(crate::prelude::BlockPos::from(data));
        assert_eq!(pos.x(), u8::try_from(data[0].rem_euclid(16)).unwrap());
        assert_eq!(pos.y(), u8::try_from(data[1].rem_euclid(16)).unwrap());
        assert_eq!(pos.z(), u8::try_from(data[2].rem_euclid(16)).unwrap());
    }
}
