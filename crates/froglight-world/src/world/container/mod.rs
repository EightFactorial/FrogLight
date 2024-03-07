mod heightmap;
use std::ops::Range;

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
            Palette::Vector(vec) => vec[value as usize],
            // Return the value directly if the palette is global.
            Palette::Global => value,
            Palette::Single(_) => unreachable!(),
        }
    }

    /// Sets the value at the given coordinates.
    ///
    /// Returns the previous value at the given coordinates.
    #[allow(clippy::missing_panics_doc)]
    pub fn set_data(&mut self, pos: &SectionBlockPosition, value: u32) -> u32 {
        match &self.palette {
            Palette::Single(v) => {
                if *v == value {
                    // Do nothing, the value is already set.
                    value
                } else {
                    // Store the old value for later.
                    let old_value = *v;

                    // Convert the palette to a vector and add the new value.
                    self.palette = Palette::Vector(vec![*v, value]);

                    // Set the bitsize to 1.
                    self.bits = 1;

                    // Create a new empty bitvec
                    let mut data = BitVec::repeat(false, Self::data_size(self.bits));

                    // Set the new value in the bitslice.
                    let mut_slice = &mut data[Self::entry_range(self.bits, *pos)];
                    mut_slice.set(0, true);

                    // Store the new data.
                    self.data = data;

                    // Return the old value.
                    old_value
                }
            }
            Palette::Vector(vec) => {
                if let Some(index) = vec.iter().position(|&v| v == value) {
                    // TODO: Borrow checker >:(
                    let vec = vec.clone();

                    // Get the bitslice mutably and retrieve the existing index.
                    let slice = self.get_bitslice_mut(*pos);
                    let old_index = slice.load_be::<usize>();

                    if let Some(old_value) = vec.get(old_index) {
                        // Store the new index in the bitslice.
                        slice.store_be(index);
                        // Return the existing value.
                        *old_value
                    } else {
                        // Log an error and return 0 (Air).
                        bevy_log::error!(
                            "Existing value in Palette::Vector does not exist in palette!"
                        );
                        0
                    }
                } else {
                    todo!("Add value to palette, possibly expand the bitsize or convert to Palette::Global")
                }
            }
            Palette::Global => {
                // Get the bitslice mutably and retrieve the existing value.
                let slice = self.get_bitslice_mut(*pos);
                let old_value = slice.load_be::<u32>();

                // Store the new value in the bitslice.
                slice.store_be(value);

                // Return the existing value.
                old_value
            }
        }
    }
}

/// Bitslice and calculation methods for accessing
/// the data in a [`ChunkDataContainer`].
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
    fn entry_range(bits: usize, pos: SectionBlockPosition) -> Range<usize> {
        let start = Self::entry_start(bits, pos);
        start..start + bits
    }

    /// Returns the start position of the entry in bits.
    #[must_use]
    fn entry_start(bits: usize, pos: SectionBlockPosition) -> usize {
        let pos_index = pos.as_index();
        let entries_per_long = Self::entries_per_long(bits);

        let long_index = pos_index / entries_per_long;
        let long_offset = pos_index % entries_per_long;

        (long_index * Self::U64BITS) + (Self::U64BITS - (long_offset * bits)) - bits
    }

    /// Returns the total length of all entries in bits.
    const fn data_size(bits: usize) -> usize {
        let entries_per_long = Self::entries_per_long(bits);
        Section::VOLUME.div_ceil(entries_per_long) * Self::U64BITS
    }

    /// Returns the number of entries that can be stored in a single long.
    #[inline]
    const fn entries_per_long(bits: usize) -> usize { Self::U64BITS / bits }
}

#[test]
fn empty_container() {
    let container = ChunkDataContainer::<BlockContainer>::default();

    // Check that the container is empty.
    assert_eq!(container.bits, 0);
    assert_eq!(container.palette, Palette::Single(0));
    assert_eq!(container.data, BitVec::<u64, Msb0>::EMPTY);

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

    // Set the value at the given position.
    let set_pos = SectionBlockPosition::new(2, 4, 8);
    let value = 5;

    // Set the value and check that it's set.
    assert_eq!(container.set_data(&set_pos, value), 0);
    assert_eq!(container.get_data(&set_pos), value);

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
fn wiki_example() {
    let container = ChunkDataContainer::<BlockContainer> {
        bits: 5,
        palette: Palette::Vector(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ]),
        data: BitVec::from_slice(&[
            0b0000_0000_0010_0000_1000_0110_0011_0001_0100_1000_0100_0001_1000_1000_0100_0001,
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
