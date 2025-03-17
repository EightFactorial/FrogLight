use std::{marker::PhantomData, ops::Range};

use bitvec::{field::BitField, order::LocalBits, slice::BitSlice, vec::BitVec};
#[cfg(feature = "io")]
use froglight_io::prelude::*;

use super::SectionPalette;
use crate::position::SectionBlockPos;

/// A cube of block and biome data.
///
/// Contains both block and biome data.
#[derive(Default, Clone)]
#[cfg_attr(feature = "io", derive(FrogBuf))]
pub struct Section {
    /// The number of non-air blocks in the section.
    solid: u32,
    /// Binary block data.
    block: SectionData<Block>,
    /// Binary biome data.
    biome: SectionData<Biome>,
}

impl Section {
    /// The depth of a [`Section`] in blocks.
    pub const DEPTH: usize = Self::HEIGHT;
    /// The height of a [`Section`] in blocks.
    pub const HEIGHT: usize = 16;
    /// The volume of a [`Section`] in blocks.
    pub const VOLUME: usize = Self::DEPTH * Self::HEIGHT * Self::WIDTH;
    /// The width of a [`Section`] in blocks.
    pub const WIDTH: usize = Self::HEIGHT;

    /// Get the number of non-air blocks in the [`Section`].
    #[inline]
    #[must_use]
    pub const fn blocks(&self) -> u32 { self.solid }

    /// Get the number of non-air blocks in the [`Section`] mutably.
    #[inline]
    #[must_use]
    pub const fn blocks_mut(&mut self) -> &mut u32 { &mut self.solid }

    /// Get the raw block [`SectionData`].
    #[inline]
    #[must_use]
    pub const fn blocks_raw(&self) -> &SectionData<Block> { &self.block }

    /// Get the block id at the given block index.
    #[must_use]
    pub fn get_block(&self, pos: SectionBlockPos) -> u32 { self.block.get(pos.into_index()) }

    /// Set the block id at the given block index.
    ///
    /// Returns the previous block id.
    ///
    /// # Warning
    /// This does not update the block count! This ***must*** be done manually!
    pub fn set_block(&mut self, pos: SectionBlockPos, block_id: u32) -> u32 {
        self.block.set(pos.into_index(), block_id)
    }

    /// Get the raw biome [`SectionData`].
    #[inline]
    #[must_use]
    pub const fn biomes_raw(&self) -> &SectionData<Biome> { &self.biome }

    /// Get the biome id at the given block index.
    #[must_use]
    pub fn get_biome(&self, pos: SectionBlockPos) -> u32 { self.biome.get(pos.into_index()) }

    /// Set the biome id at the given block index.
    ///
    /// Returns the previous biome id.
    pub fn set_biome(&mut self, pos: SectionBlockPos, biome_id: u32) -> u32 {
        self.biome.set(pos.into_index(), biome_id)
    }
}

// -------------------------------------------------------------------------------------------------

/// A bit-packed cube of world data.
///
/// Contains either [`Block`] or [`Biome`] data.
#[derive(Default, Clone)]
#[expect(private_bounds)]
pub struct SectionData<T: SectionType> {
    bits: usize,
    palette: SectionPalette,
    data: BitVec<u64, LocalBits>,
    _phantom: PhantomData<T>,
}

#[expect(private_bounds)]
impl<T: SectionType> SectionData<T> {
    /// Create a new [`SectionData`] with
    /// the given number of bits, the palette, and the section data.
    #[must_use]
    pub const fn new(bits: usize, palette: SectionPalette, data: BitVec<u64, LocalBits>) -> Self {
        Self { bits, palette, data, _phantom: PhantomData }
    }

    /// Get the number of bits used to store each entry.
    #[inline]
    #[must_use]
    pub const fn bits(&self) -> usize { self.bits }

    /// Get the number of bits used to store each entry mutably.
    #[inline]
    #[must_use]
    pub const fn bits_mut(&mut self) -> &mut usize { &mut self.bits }

    /// Get the palette used to encode and decode the data.
    #[inline]
    #[must_use]
    pub const fn palette(&self) -> &SectionPalette { &self.palette }

    /// Get the palette used to encode and decode the data mutably.
    #[inline]
    #[must_use]
    pub const fn palette_mut(&mut self) -> &mut SectionPalette { &mut self.palette }

    /// Get the raw section data.
    #[inline]
    #[must_use]
    pub const fn raw_data(&self) -> &BitVec<u64, LocalBits> { &self.data }

    /// Get the raw section data mutably.
    #[inline]
    #[must_use]
    pub const fn raw_data_mut(&mut self) -> &mut BitVec<u64, LocalBits> { &mut self.data }
}

#[expect(private_bounds)]
impl<T: SectionType> SectionData<T> {
    /// Get the value at the given index.
    ///
    /// # Panics
    /// Panics if the index is over `4096`.
    #[must_use]
    pub fn get(&self, index: usize) -> u32 {
        match self.palette() {
            SectionPalette::Single(item) => *item,
            SectionPalette::Vector(items) => items[self.slice_at(index).load::<usize>()],
            SectionPalette::Global => self.slice_at(index).load::<u32>(),
        }
    }

    /// Set the value at the given index.
    ///
    /// # Panics
    /// Panics if the index is over `4096`.
    pub fn set(&mut self, index: usize, value: u32) -> u32 {
        let previous = self.get(index);

        match self.palette() {
            SectionPalette::Single(item) => {
                let item = *item;
                // Upgrade to a `SectionPalette::Vector` if the new value is different.
                if item != value {
                    *self.bits_mut() = 1;
                    *self.palette_mut() = SectionPalette::Vector(vec![item, value]);
                    *self.raw_data_mut() = BitVec::repeat(false, Section::VOLUME);
                    // Set the new value.
                    self.slice_at_mut(index).store(value);
                }
            }
            SectionPalette::Vector(items) => {
                // If the value is not in the palette, add it.
                if let Some(value_index) = items.iter().position(|&item| item == value) {
                    // Set the new value in the data.
                    self.slice_at_mut(index).store(value_index);
                } else {
                    // Get the index of the new value.
                    let mut items = items.clone();
                    let value_index = items.len();

                    // Expand the palette if necessary.
                    let required = usize::BITS - items.len().leading_zeros();
                    match T::palette_for(required as usize) {
                        // Expand the palette to the required number of bits.
                        SectionPalette::Single(..) | SectionPalette::Vector(..) => {
                            self.expand_palette_to(required);

                            // Set the new value in the data.
                            self.slice_at_mut(index).store(value_index);

                            // Add the new value to the palette.
                            items.push(value);
                            *self.palette_mut() = SectionPalette::Vector(items);
                        }
                        // Convert the data to use the global palette.
                        SectionPalette::Global => self.convert_global_palette(value_index, value),
                    }
                }
            }
            SectionPalette::Global => {
                // Expand the palette if necessary.
                self.expand_palette_to(u32::BITS - value.leading_zeros());

                // Set the new value in the data.
                self.slice_at_mut(index).store(value);
            }
        }

        // Return the previous value.
        previous
    }

    /// Expand the palette to the given number of bits.
    ///
    /// Does nothing if the palette is already large enough.
    fn expand_palette_to(&mut self, required: u32) {
        let required = required as usize;
        if self.bits() >= required {
            return;
        }

        let mut new_data = BitVec::repeat(false, Section::VOLUME * required);
        {
            for index in 0..Section::VOLUME {
                let old = Self::slice_of(self.bits, &self.data, index);
                let new = Self::slice_of_mut(required, &mut new_data, index);
                new[0..required - 1].copy_from_bitslice(old);
            }
        }

        // Set the new bits and data.
        *self.bits_mut() = required;
        *self.raw_data_mut() = new_data;
    }

    /// Convert the palette to a global palette.
    ///
    /// # Panics
    /// Panics if the [`SectionPalette`] is not a [`SectionPalette::Vector`].
    fn convert_global_palette(&mut self, index: usize, next: u32) {
        // Take the items out of the palette.
        let SectionPalette::Vector(items) =
            std::mem::replace(self.palette_mut(), SectionPalette::Global)
        else {
            unreachable!("Only `Vector` palettes can be converted to `Global`!")
        };

        let max = items.iter().max().unwrap().max(&next).leading_zeros();
        let required = (u32::BITS - max) as usize;

        // Expand the data to the required number of bits,
        // while converting the data to use a global palette.
        let mut new_data = BitVec::repeat(false, Section::VOLUME * required);
        {
            for index in 0..Section::VOLUME {
                let new = Self::slice_of_mut(required, &mut new_data, index);
                let old = self.slice_at(index);
                new[0..required - 1].store(items[old.load::<usize>()]);
            }
        }

        // Set the next value in the data.
        Self::slice_of_mut(required, &mut new_data, index).store(next);

        // Set the new bits and data.
        *self.bits_mut() = required;
        *self.raw_data_mut() = new_data;
    }

    /// Get a reference to the entry at the given index.
    ///
    /// # Panics
    /// Panics if the index is over `4096`,
    /// or if the [`SectionPalette`] is [`SectionPalette::Single`]
    #[inline]
    #[must_use]
    pub fn slice_at(&self, index: usize) -> &BitSlice<u64, LocalBits> {
        Self::slice_of(self.bits, &self.data, index)
    }

    /// Get a mutable reference to the entry at the given index.
    ///
    /// # Panics
    /// Panics if the index is over `4096`,
    /// or if the [`SectionPalette`] is [`SectionPalette::Single`]
    #[inline]
    #[must_use]
    pub fn slice_at_mut(&mut self, index: usize) -> &mut BitSlice<u64, LocalBits> {
        Self::slice_of_mut(self.bits, &mut self.data, index)
    }

    /// Get a reference to the entry at the given index.
    ///
    /// # Panics
    /// Panics if the index is over `4096`,
    /// or if the [`SectionPalette`] is [`SectionPalette::Single`]
    #[must_use]
    fn slice_of(
        bits: usize,
        data: &BitVec<u64, LocalBits>,
        index: usize,
    ) -> &BitSlice<u64, LocalBits> {
        debug_assert!(index < Section::VOLUME);

        let range = Self::slice_range(bits, index);
        data.get(range).unwrap_or_else(|| {
            unreachable!("SectionData always expands to handle valid indices!");
        })
    }

    /// Get a mutable reference to the entry at the given index.
    ///
    /// # Panics
    /// Panics if the index is over `4096`,
    /// or if the [`SectionPalette`] is [`SectionPalette::Single`]
    #[must_use]
    fn slice_of_mut(
        bits: usize,
        data: &mut BitVec<u64, LocalBits>,
        index: usize,
    ) -> &mut BitSlice<u64, LocalBits> {
        debug_assert!(index < Section::VOLUME);

        let range = Self::slice_range(bits, index);
        data.get_mut(range).unwrap_or_else(|| {
            unreachable!("SectionData always expands to handle valid indices!");
        })
    }

    /// Get the range of bits for the given index.
    #[inline]
    const fn slice_range(bits: usize, index: usize) -> Range<usize> {
        let start = index * bits;
        Range { start, end: start + bits }
    }
}

#[test]
fn slice() {
    assert_eq!(SectionData::<Block>::slice_range(1, 0), 0..1);
    assert_eq!(SectionData::<Block>::slice_range(1, 1), 1..2);
    assert_eq!(SectionData::<Block>::slice_range(1, 2), 2..3);
    assert_eq!(SectionData::<Block>::slice_range(1, 3), 3..4);

    assert_eq!(SectionData::<Block>::slice_range(2, 0), 0..2);
    assert_eq!(SectionData::<Block>::slice_range(2, 1), 2..4);
    assert_eq!(SectionData::<Block>::slice_range(2, 2), 4..6);
    assert_eq!(SectionData::<Block>::slice_range(2, 3), 6..8);

    assert_eq!(SectionData::<Block>::slice_range(15, 0), 0..15);
    assert_eq!(SectionData::<Block>::slice_range(15, 1), 15..30);
    assert_eq!(SectionData::<Block>::slice_range(15, 2), 30..45);
    assert_eq!(SectionData::<Block>::slice_range(15, 3), 45..60);

    assert_eq!(SectionData::<Block>::slice_range(16, 0), 0..16);
    assert_eq!(SectionData::<Block>::slice_range(16, 1), 16..32);
    assert_eq!(SectionData::<Block>::slice_range(16, 2), 32..48);
    assert_eq!(SectionData::<Block>::slice_range(16, 3), 48..64);
}

// -------------------------------------------------------------------------------------------------

#[derive(Default, Clone, Copy)]
pub struct Biome;

impl SectionType for Biome {
    fn palette_for(bits: usize) -> SectionPalette {
        match bits {
            0 => SectionPalette::Single(0u32),
            1..=3 => SectionPalette::Vector(Vec::new()),
            _ => SectionPalette::Global,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Block;

impl SectionType for Block {
    fn palette_for(bits: usize) -> SectionPalette {
        match bits {
            0 => SectionPalette::Single(0u32),
            1..=8 => SectionPalette::Vector(Vec::new()),
            _ => SectionPalette::Global,
        }
    }
}

use sealed::SectionType;
mod sealed {
    #![allow(dead_code)]
    #[expect(clippy::wildcard_imports)]
    use super::*;

    /// A type of [`Section`] storage.
    pub(crate) trait SectionType: Default + Clone + Send + Sync + 'static {
        /// Get a [`SectionPalette`] for this number of bits.
        fn palette_for(bits: usize) -> SectionPalette;

        /// Read a [`SectionPalette`] from a buffer, given the number of bits.
        #[cfg(feature = "io")]
        fn read_palette(
            bits: usize,
            buffer: &mut impl std::io::Read,
        ) -> Result<SectionPalette, ReadError> {
            Self::palette_for(bits).frog_read(buffer)
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl<T: SectionType> FrogRead for SectionData<T> {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        let bits = u8::frog_read(buffer)? as usize;
        let palette = T::read_palette(bits, buffer)?;
        let data = BitVec::from_vec(Vec::<u64>::frog_read(buffer)?);
        Ok(Self { bits, palette, data, _phantom: PhantomData })
    }
}

#[cfg(feature = "io")]
impl<T: SectionType> FrogWrite for SectionData<T> {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        let mut len = 0;
        len += u8::frog_write(&self.bits.try_into().unwrap(), buffer)?;
        len += self.palette.frog_write(buffer)?;
        len += self.data.as_raw_slice().frog_write(buffer)?;
        Ok(len)
    }

    fn frog_len(&self) -> usize {
        u8::frog_len(&0) + self.palette.frog_len() + self.data.as_raw_slice().frog_len()
    }
}
