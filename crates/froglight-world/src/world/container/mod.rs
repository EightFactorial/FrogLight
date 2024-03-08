mod heightmap;
use std::ops::Range;

use bevy_log::error;
use bitvec::{field::BitField, order::Msb0, slice::BitSlice, vec::BitVec};
use froglight_core::common::SectionBlockPosition;
pub use heightmap::*;

mod chunk;
pub use chunk::*;

use super::{Palette, Section};

/// Getting and setting data in a [`ChunkDataContainer`].
impl<T: ContainerType> ChunkDataContainer<T> {
    /// Gets the value at the given coordinates.
    ///
    /// # Panics
    /// Panics if the palette value is out of range.
    #[must_use]
    pub fn get_data(&self, pos: &SectionBlockPosition) -> u32 {
        // Skip the lookup if the palette only contains a single value.
        if let Palette::Single(v) = self.palette {
            return v;
        }

        // Load the value from the bitslice and convert it to a usize.
        let slice = self.get_bitslice(*pos);
        let value = slice.load_be::<u32>();

        match &self.palette {
            // Get the value from the palette if it's a vector.
            Palette::Vector(vec) => {
                if let Some(value) = vec.get(value as usize) {
                    *value
                } else {
                    error!("Value in BitVec does not exist in Palette::Vector!");
                    0
                }
            }
            // Return the value directly if the palette is global.
            Palette::Global => value,
            Palette::Single(_) => {
                unreachable!("Palette::Single was handled earlier")
            }
        }
    }

    /// Sets the value at the given coordinates.
    ///
    /// Returns the previous value.
    #[allow(clippy::missing_panics_doc)]
    pub fn set_data(&mut self, pos: &SectionBlockPosition, value: u32) -> u32 {
        match &self.palette {
            Palette::Single(_) => self.set_single(*pos, value),
            Palette::Vector(_) => self.set_vector(*pos, value),
            Palette::Global => self.set_global(*pos, value),
        }
    }

    /// Sets the value for [`Palette::Single`] palettes.
    fn set_single(&mut self, pos: SectionBlockPosition, value: u32) -> u32 {
        let Palette::Single(v) = &self.palette else {
            unreachable!("Palette must be Palette::Single");
        };

        if *v == value {
            // Do nothing, the value is already set.
            value
        } else {
            // Store the old value to return later.
            let old_value = *v;

            // Convert the palette to a vector and add the new value.
            self.palette = Palette::Vector(vec![*v, value]);

            // Set the bitsize to 1.
            self.bits = 1;

            // Create a new empty bitvec
            let mut data = BitVec::repeat(false, Self::data_size_bits(self.bits));

            // Set the new value in the bitslice.
            let mut_slice = &mut data[Self::entry_range(self.bits, pos)];
            mut_slice.set(0, true);

            // Store the new data.
            self.data = data;

            // Return the old value.
            old_value
        }
    }

    /// Sets the value for [`Palette::Global`] palettes.
    fn set_vector(&mut self, pos: SectionBlockPosition, value: u32) -> u32 {
        let Palette::Vector(vec) = &self.palette else {
            unreachable!("Palette must be Palette::Vector");
        };

        if let Some(index) = vec.iter().position(|&v| v == value) {
            // TODO: Borrow checker >:(
            let vec = vec.clone();

            // Get the bitslice mutably and retrieve the existing index.
            let slice = self.get_bitslice_mut(pos);
            let old_index = slice.load_be::<usize>();

            if let Some(old_value) = vec.get(old_index) {
                // Store the new index in the bitslice.
                slice.store_be(index);
                // Return the existing value.
                *old_value
            } else {
                // Log an error and return 0 (Usually air).
                error!("Value in BitVec does not exist in Palette::Vector!");
                0
            }
        } else {
            // TODO: Borrow checker >:(
            let mut vec = vec.clone();

            // Check if the palette needs to be expanded.
            let required_size = value.next_power_of_two().trailing_zeros() as usize - 1;
            match T::palette_type(required_size) {
                Palette::Vector(_) => {
                    // Expand the bitvec to fit the new value.
                    if required_size > self.bits {
                        self.expand_bitvec_by(required_size - self.bits);
                    }

                    // Add the value to the palette.
                    let new_index = vec.len();
                    vec.push(value);

                    // Set the new palette.
                    self.palette = Palette::Vector(vec.clone());

                    // Get the bitslice mutably and retrieve the existing index.
                    let slice = self.get_bitslice_mut(pos);
                    let old_index = slice.load_be::<usize>();

                    // Store the new index in the bitslice.
                    slice.store_be(new_index);

                    // Return the existing value.
                    if let Some(&old_value) = vec.get(old_index) {
                        old_value
                    } else {
                        // Log an error and return 0 (Usually air).
                        error!("Value in BitVec does not exist in Palette::Vector!");
                        0
                    }
                }
                Palette::Global => {
                    // Convert the palette to a global palette.
                    self.convert_to_global(required_size);

                    // Set the value in the global palette.
                    self.set_global(pos, value)
                }
                Palette::Single(_) => {
                    unreachable!("Cannot create a Palette::Single from a Palette::Vector")
                }
            }
        }
    }

    /// Sets the value for [`Palette::Global`] palettes.
    fn set_global(&mut self, pos: SectionBlockPosition, value: u32) -> u32 {
        let Palette::Global = &self.palette else {
            unreachable!("Palette must be Palette::Global");
        };

        // Check if the palette needs to be expanded.
        let required_size = (u32::BITS - value.leading_zeros()) as usize;
        if required_size > self.bits {
            // Expand the bitvec to fit the new value.
            self.expand_bitvec_by(required_size - self.bits);
        }

        // Get the bitslice mutably and retrieve the existing value.
        let slice = self.get_bitslice_mut(pos);
        let old_value = slice.load_be::<u32>();

        // Store the new value in the bitslice.
        slice.store_be(value);

        // Return the existing value.
        old_value
    }

    /// Converts the palette from [`Palette::Vector`] to [`Palette::Global`].
    fn convert_to_global(&mut self, _bits: usize) {
        todo!("Convert Palette::Global to Palette::Vector");
    }

    /// Expands the bitvec by the given number of bits.
    fn expand_bitvec_by(&mut self, bits: usize) {
        // Calculate the new size.
        let new_bits = self.bits + bits;

        // Create a new bitvec with the new larger size.
        let mut new_data = BitVec::repeat(false, Self::data_size_bits(new_bits));

        // Copy the old data into the new bitvec.
        for index in 0..Section::VOLUME {
            let pos = SectionBlockPosition::from_index(index);

            // Get the original bitslice
            let old_slice = self.get_bitslice(pos);

            // Get the new bitslice and match the size.
            let new_slice = &mut new_data[Self::entry_range(new_bits, pos)];
            let smaller_slice = &mut new_slice[bits..];

            // Copy the old bitslice into the new bitslice.
            smaller_slice.copy_from_bitslice(old_slice);
        }

        // Update the bits and data.
        self.bits = new_bits;
        self.data = new_data;
    }

    /// Compresses the bitvec and palette to use the smallest possible size
    /// while still maintaining the same data.
    ///
    /// Warning: This is an expensive operation and should be used sparingly.
    pub fn compress(&mut self) {
        match &self.palette {
            Palette::Single(_) => {
                // Do nothing, the bitvec is already as small as possible.
            }
            Palette::Vector(_) => {
                todo! {
                    "Check for empty Palette indexes and remove them.
                    If there is only one value, convert to Palette::Single.
                    Reduce the bitsize if possible"
                };
            }
            Palette::Global => {
                todo! {
                    "Find the largest value and get the bitsize,
                    Potentially compress back into a Palette::Vector or Palette::Single"
                };
            }
        }
    }
}

/// Bitslice and calculation methods for accessing
/// data in a [`ChunkDataContainer`].
impl<T: ContainerType> ChunkDataContainer<T> {
    const U64BITS: usize = u64::BITS as usize;

    /// Returns a [`BitSlice`] for the given position.
    #[must_use]
    #[inline]
    pub fn get_bitslice(&self, pos: SectionBlockPosition) -> &BitSlice<u64, Msb0> {
        &self.data[Self::entry_range(self.bits, pos)]
    }

    /// Returns a mutable [`BitSlice`] for the given position.
    #[must_use]
    #[inline]
    pub fn get_bitslice_mut(&mut self, pos: SectionBlockPosition) -> &mut BitSlice<u64, Msb0> {
        &mut self.data[Self::entry_range(self.bits, pos)]
    }

    /// Returns the range of bits that the entry is stored in.
    #[must_use]
    #[inline]
    const fn entry_range(bits: usize, pos: SectionBlockPosition) -> Range<usize> {
        let start = Self::entry_start(bits, pos);
        Range { start, end: start + bits }
    }

    /// Returns the start position of the entry in bits.
    #[must_use]
    const fn entry_start(bits: usize, pos: SectionBlockPosition) -> usize {
        let entries_per_long = Self::entries_per_long(bits);
        let pos_index = pos.as_index();

        let long_index = pos_index / entries_per_long;
        let long_offset = pos_index % entries_per_long;

        // {     Find the long     }   {      Find the bit index in the long       }
        (long_index * Self::U64BITS) + (Self::U64BITS - (long_offset * bits)) - bits
    }

    /// Returns the number of entries that can be stored in a single long.
    #[must_use]
    #[inline]
    const fn entries_per_long(bits: usize) -> usize { Self::U64BITS / bits }

    /// Returns the number of bits required to store a section.
    #[must_use]
    #[inline]
    const fn data_size_bits(bits: usize) -> usize { Self::data_size_longs(bits) * Self::U64BITS }

    /// Returns the number of longs required to store a section.
    #[must_use]
    #[inline]
    const fn data_size_longs(bits: usize) -> usize {
        Section::VOLUME.div_ceil(Self::entries_per_long(bits))
    }
}

#[test]
fn data_size() {
    let sizes: [usize; 32] = [
        4096, 8192, 12544, 16384, 21888, 26240, 29184, 32768, 37504, 43712, 52480, 52480, 65536,
        65536, 65536, 65536, 87424, 87424, 87424, 87424, 87424, 131_072, 131_072, 131_072, 131_072,
        131_072, 131_072, 131_072, 131_072, 131_072, 131_072, 131_072,
    ];

    for (i, &size) in sizes.iter().enumerate() {
        assert_eq!(ChunkDataContainer::<BlockContainer>::data_size_bits(i + 1), size);
    }
}

#[test]
fn empty_container() {
    let container = ChunkDataContainer::<BlockContainer>::default();

    // Check that the container is empty.
    assert_eq!(container.bits, 0);
    assert_eq!(container.palette, Palette::Single(0));
    assert_eq!(container.data, BitVec::<u64, Msb0>::EMPTY);

    // Check that it's possible to get the first and last values.
    assert_eq!(container.get_data(&SectionBlockPosition::FIRST), 0);
    assert_eq!(container.get_data(&SectionBlockPosition::LAST), 0);

    // Check that all values are 0.
    for y in 0..u8::try_from(Section::HEIGHT).unwrap() {
        for z in 0..u8::try_from(Section::DEPTH).unwrap() {
            for x in 0..u8::try_from(Section::WIDTH).unwrap() {
                let pos = SectionBlockPosition::new(x, y, z);
                assert_eq!(container.get_data(&pos), 0);
            }
        }
    }
}

#[test]
fn single_container() {
    let mut container = ChunkDataContainer::<BlockContainer>::default();
    assert_eq!(container.bits, 0);

    // Check that it's possible to get the first and last values.
    assert_eq!(container.get_data(&SectionBlockPosition::FIRST), 0);
    assert_eq!(container.get_data(&SectionBlockPosition::LAST), 0);

    // Set the value at the given position.
    let set_pos = SectionBlockPosition::new(2, 4, 8);
    let value = 5;

    // Set the value and check that it's set.
    assert_eq!(container.set_data(&set_pos, value), 0);
    assert_eq!(container.get_data(&set_pos), value);
    assert_eq!(container.bits, 1);

    // Check that it's possible to get the first and last values.
    assert_eq!(container.get_data(&SectionBlockPosition::FIRST), 0);
    assert_eq!(container.get_data(&SectionBlockPosition::LAST), 0);

    // Check that all other values are 0.
    for y in 0..u8::try_from(Section::HEIGHT).unwrap() {
        for z in 0..u8::try_from(Section::DEPTH).unwrap() {
            for x in 0..u8::try_from(Section::WIDTH).unwrap() {
                let pos = SectionBlockPosition::new(x, y, z);
                if pos != set_pos {
                    assert_eq!(container.get_data(&pos), 0);
                }
            }
        }
    }

    // Set the value again and make sure nothing changed.
    assert_eq!(container.set_data(&set_pos, value), value);
    assert_eq!(container.get_data(&set_pos), value);
    assert_eq!(container.bits, 1);

    // Check that it's possible to get the first and last values.
    assert_eq!(container.get_data(&SectionBlockPosition::FIRST), 0);
    assert_eq!(container.get_data(&SectionBlockPosition::LAST), 0);

    // Check that all other values are 0.
    for y in 0..u8::try_from(Section::HEIGHT).unwrap() {
        for z in 0..u8::try_from(Section::DEPTH).unwrap() {
            for x in 0..u8::try_from(Section::WIDTH).unwrap() {
                let pos = SectionBlockPosition::new(x, y, z);
                if pos != set_pos {
                    assert_eq!(container.get_data(&pos), 0);
                }
            }
        }
    }
}

#[test]
fn vector_container() {
    let mut container = ChunkDataContainer::<BlockContainer>::default();
    assert_eq!(container.bits, 0);

    // Check that it's possible to get the first and last values.
    assert_eq!(container.get_data(&SectionBlockPosition::FIRST), 0);
    assert_eq!(container.get_data(&SectionBlockPosition::LAST), 0);

    // Create a position and value to set.
    let first_pos = SectionBlockPosition::new(0, 0, 0);
    let first_val = 5;

    // Set the value and check that it's set.
    assert_eq!(container.set_data(&first_pos, first_val), 0);
    assert_eq!(container.get_data(&first_pos), first_val);
    assert_eq!(container.bits, 1);

    // Check that it's possible to get the first and last values.
    assert_eq!(container.get_data(&SectionBlockPosition::FIRST), 5);
    assert_eq!(container.get_data(&SectionBlockPosition::LAST), 0);

    // Create a second position and value to set.
    let second_pos = SectionBlockPosition::new(1, 0, 0);
    let second_val = 6;

    // Get the first value and check that it's still set.
    assert_eq!(container.get_data(&first_pos), first_val);
    assert_eq!(container.bits, 1);
    // Set the second value and check that it's set.
    assert_eq!(container.set_data(&second_pos, second_val), 0);
    assert_eq!(container.get_data(&second_pos), second_val);
    assert_eq!(container.bits, 2);

    // Check that it's possible to get the first and last values.
    assert_eq!(container.get_data(&SectionBlockPosition::FIRST), 5);
    assert_eq!(container.get_data(&SectionBlockPosition::LAST), 0);

    // Check that all other values are 0.
    for y in 0..u8::try_from(Section::HEIGHT).unwrap() {
        for z in 0..u8::try_from(Section::DEPTH).unwrap() {
            for x in 0..u8::try_from(Section::WIDTH).unwrap() {
                let pos = SectionBlockPosition::new(x, y, z);
                if pos != first_pos && pos != second_pos {
                    assert_eq!(container.get_data(&pos), 0);
                }
            }
        }
    }

    // Create a third position and value to set.
    let third_pos = SectionBlockPosition::new(2, 0, 0);
    let third_val = 7;

    // Get the first and second values and check that they're still set.
    assert_eq!(container.get_data(&first_pos), first_val);
    assert_eq!(container.get_data(&second_pos), second_val);
    // Set the third value and check that it's set.
    assert_eq!(container.set_data(&third_pos, third_val), 0);
    assert_eq!(container.get_data(&third_pos), third_val);
    assert_eq!(container.bits, 2);

    // Check that it's possible to get the first and last values.
    assert_eq!(container.get_data(&SectionBlockPosition::FIRST), 5);
    assert_eq!(container.get_data(&SectionBlockPosition::LAST), 0);

    // Check that all other values are 0.
    for y in 0..u8::try_from(Section::HEIGHT).unwrap() {
        for z in 0..u8::try_from(Section::DEPTH).unwrap() {
            for x in 0..u8::try_from(Section::WIDTH).unwrap() {
                let pos = SectionBlockPosition::new(x, y, z);
                if pos != first_pos && pos != second_pos && pos != third_pos {
                    assert_eq!(container.get_data(&pos), 0);
                }
            }
        }
    }
}

#[test]
fn wiki_example() {
    let container = ChunkDataContainer::<BlockContainer> {
        bits: 5,
        palette: Palette::Vector(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ]),
        data: BitVec::from_slice(&[
            //                                       ...{  4 }{  4 } {  3 }{  2 }{  2 }{  1 }
            0b0000_0000_0010_0000_1000_0110_0011_0001_0100_1000_0100_0001_1000_1000_0100_0001,
            //                                       ...{ 16 }{ 15 } { 13 }{  3 }{  4 }{  7 }
            0b0000_0001_0000_0001_1000_1010_0111_0010_0110_0000_1111_0110_1000_1100_1000_0111,
        ]),
        _phantom: std::marker::PhantomData,
    };

    for (i, n) in [1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 2u32]
        .into_iter()
        .enumerate()
    {
        assert_eq!(container.get_data(&SectionBlockPosition::from_index(i)), n);
    }
}

// #[test]
// fn global_container() {
//     let mut container = ChunkDataContainer::<BlockContainer>::default();
//     assert_eq!(container.bits, 0);
// }
