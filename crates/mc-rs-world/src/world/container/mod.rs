use std::{cmp::Ordering, io::Cursor};

use super::{palette::Palette, section::Section, tasks::ChunkDecodeError};

pub mod traits;
use bevy::log::error;
use bitvec::{field::BitField, order::Msb0, slice::BitSlice, vec::BitVec};
use mc_rs_core::position::ChunkBlockPos;
use traits::ContainerType;

mod biomes;
pub use biomes::BiomeContainer;

mod blocks;
pub use blocks::BlockContainer;

/// A [`Container`] is used to store a type of data in a [`Section`].
///
/// For [`BlockContainer`], it contains block data.
///
/// For [`BiomeContainer`], it contains biome data.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Container<T: ContainerType> {
    pub bits: usize,
    pub palette: Palette,
    pub data: BitVec<u64, Msb0>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: ContainerType> Container<T> {
    pub(super) async fn decode_container(
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Self, ChunkDecodeError> {
        T::decode_container(cursor).await
    }
}

impl<T: ContainerType> Container<T> {
    /// Gets the value at the given block position
    pub fn get(&self, pos: &ChunkBlockPos) -> u32 {
        if let Palette::Single(n) = self.palette {
            return n;
        }

        let bitslice = self.get_bitslice(pos);
        let bitslice_value = bitslice.load_be::<usize>();

        if let Palette::Global = self.palette {
            bitslice_value.try_into().expect("Block ID out of range")
        } else if let Some(value) = self.palette.get(bitslice_value) {
            *value
        } else {
            error!(
                "Invalid Palette index: `{bitslice_value}` for Palette::{:?}",
                self.palette
            );
            u32::MAX
        }
    }

    /// Sets the value at the given block position
    pub fn set(&mut self, value: u32, pos: &ChunkBlockPos) {
        match &mut self.palette {
            Palette::Single(n) => {
                if *n != value {
                    self.palette = Palette::Vector(vec![*n, value]);
                    self.bits = 1;

                    let mut data = BitVec::repeat(false, Self::bitsize_to_data_length(1));
                    data.set(Self::block_pos_to_bit_index(1, pos), true);

                    self.data = data;
                }
            }
            Palette::Vector(n) => {
                if let Some(index) = n.iter().position(|&x| x == value) {
                    let bitslice = self.get_bitslice_mut(pos);
                    bitslice.store_be(index);

                    // TODO: Check if we can decrease the bits to compress the data
                } else {
                    let index = n.len();
                    n.push(value);

                    // Check if we need to increase the bits to address the new value in the palette
                    if index >= (1 << self.bits) {
                        // Convert to Global palette if necessary
                        if let Palette::Global = T::palette_type(self.bits + 1) {
                            self.vector_to_global();
                        } else {
                            // Expand the container by one bit
                            self.expand(1);
                        }
                    }

                    // Store the new value
                    let bitslice = self.get_bitslice_mut(pos);
                    bitslice.store_be(index);
                }
            }
            Palette::Global => {
                // Check if we need to increase the bits to store the new value
                let value_bits = (u32::BITS - value.leading_zeros()) as usize;
                match self.bits.cmp(&value_bits) {
                    Ordering::Less => {
                        // Expand the container by the required amount
                        self.expand(value_bits - self.bits);

                        // Store the new value
                        let bitslice = self.get_bitslice_mut(pos);
                        bitslice.store_be(value);
                    }
                    Ordering::Equal => {
                        // Store the new value
                        let bitslice = self.get_bitslice_mut(pos);
                        bitslice.store_be(value);
                    }
                    Ordering::Greater => {
                        // Store the new value
                        let bitslice = self.get_bitslice_mut(pos);
                        bitslice.store_be(value);

                        // TODO: Check if we can decrease the bits to compress the data
                    }
                }
            }
        }
    }

    /// Expands the container by the given number of bits
    fn expand(&mut self, count: usize) {
        let new_bits = self.bits + count;

        // Copy old data to new data, leaving zeros in the new bits
        let mut new_data = BitVec::repeat(false, Self::bitsize_to_data_length(new_bits));
        for pos in all_positions() {
            let old_index = Self::block_pos_to_bit_index(self.bits, &pos);
            let old_bitslice = &self.data[old_index..old_index + self.bits];

            let new_index = Self::block_pos_to_bit_index(new_bits, &pos);
            let new_bitslice = &mut new_data[new_index + count..new_index + new_bits];

            new_bitslice.copy_from_bitslice(old_bitslice);
        }

        self.bits = new_bits;
        self.data = new_data;
    }

    /// Converts the palette from vector to global
    fn vector_to_global(&mut self) {
        let Palette::Vector(n) = self.palette.clone() else {
            panic!("Only a Vector Palette can be converted to Global!")
        };

        // Find the maximum value in the palette and calculate the new bitsize
        let max_value = *n.iter().max().expect("Palette is empty!") as f32;
        let new_bits = (max_value.log2().ceil() as usize + 1).max(self.bits);

        // Create a new data array with the new bitsize
        let mut new_data = BitVec::repeat(false, Self::bitsize_to_data_length(new_bits));

        // Copy the values from the palette to the new data array
        for pos in all_positions() {
            let old_data_index = Self::block_pos_to_bit_index(self.bits, &pos);
            let old_bitslice = &self.data[old_data_index..old_data_index + self.bits];
            let value = n[old_bitslice.load_be::<usize>()];

            let new_data_index = Self::block_pos_to_bit_index(new_bits, &pos);
            let new_bitslice = &mut new_data[new_data_index..new_data_index + new_bits];

            // Write the value directly to the new data array
            new_bitslice.store_be(value);
        }

        self.bits = new_bits;
        self.data = new_data;
        self.palette = Palette::Global;
    }

    /// Gets a bitslice for the given block position
    #[inline]
    pub fn get_bitslice(&self, pos: &ChunkBlockPos) -> &BitSlice<u64, Msb0> {
        let index = Self::block_pos_to_bit_index(self.bits, pos);
        &self.data[index..index + self.bits]
    }
    /// Gets a mutable bitslice for the given block position
    #[inline]
    pub fn get_bitslice_mut(&mut self, pos: &ChunkBlockPos) -> &mut BitSlice<u64, Msb0> {
        let index = Self::block_pos_to_bit_index(self.bits, pos);
        &mut self.data[index..index + self.bits]
    }

    /// Gets the index of the first bit in the data array that corresponds to the given block
    /// position.
    ///
    /// Data is stored starting at the end of the long, until another entry cannot fit in the
    /// remaining bits at the beginning.
    ///
    /// The next entry starts at the end of the next long, and so on.
    #[inline]
    const fn block_pos_to_bit_index(bits: usize, pos: &ChunkBlockPos) -> usize {
        let block_index = pos.as_index();
        let entries_per_long = u64::BITS.div_floor(bits as u32) as usize;

        let long_index = block_index.div_floor(entries_per_long);
        let long_offset = block_index % entries_per_long;

        (long_index * u64::BITS as usize) + (u64::BITS as usize - (long_offset * bits)) - bits
    }

    /// Returns the length of the data array in bits, given the number of bits per entry
    #[inline]
    const fn bitsize_to_data_length(bits: usize) -> usize {
        let entries_per_long = u64::BITS.div_floor(bits as u32) as usize;
        let longs = Section::SECTION_VOLUME.div_ceil(entries_per_long);

        longs * u64::BITS as usize
    }
}

/// An iterator over all block positions in a section
#[inline]
fn all_positions() -> impl Iterator<Item = ChunkBlockPos> {
    (0..Section::SECTION_VOLUME).map(ChunkBlockPos::from_index)
}

#[test]
fn single_tests() {
    let mut default = Container::<BlockContainer>::default();

    // Check that the palette is filled with zeros
    for pos in all_positions() {
        assert_eq!(default.get(&pos), 0);
    }

    // Check that the default values are correct
    assert_eq!(default.palette, Palette::Single(0));
    assert_eq!(default.data, BitVec::<u64, Msb0>::EMPTY);
    assert_eq!(default.bits, 0);

    // Set a value and check that it is correct
    let some_pos = ChunkBlockPos::new(1, 2, 3);
    default.set(1, &some_pos);

    // All other values should still be zero
    for pos in all_positions() {
        if pos == some_pos {
            assert_eq!(default.get(&pos), 1);
        } else {
            assert_eq!(default.get(&pos), 0);
        }
    }

    // Check that the palette expanded correctly
    assert_eq!(default.palette, Palette::Vector(vec![0, 1]));
    assert_eq!(default.bits, 1);
}

#[test]
fn vector_tests() {
    let mut default = Container::<BlockContainer>::default();
    assert_eq!(default.palette, Palette::Single(0));
    assert_eq!(default.data, BitVec::<u64, Msb0>::EMPTY);

    const MAX_BITS: usize = 4;
    const MAX_VALUE: u32 = 1 << MAX_BITS;

    // Fill the palette with values
    for i in 0..MAX_VALUE {
        let pos = ChunkBlockPos::from_index(i as usize);
        default.set(i, &pos);
    }

    // Check that the values are correct
    for i in 0..MAX_VALUE {
        let pos = ChunkBlockPos::from_index(i as usize);
        assert_eq!(default.get(&pos), i);
    }

    // Check that the palette expanded correctly
    assert_eq!(
        default.palette,
        Palette::Vector((0..MAX_VALUE).collect::<Vec<_>>())
    );
    assert_eq!(default.bits, MAX_BITS);
}

#[test]
fn global_tests() {
    let mut default = Container::<BlockContainer>::default();
    assert_eq!(default.palette, Palette::Single(0));
    assert_eq!(default.data, BitVec::<u64, Msb0>::EMPTY);

    const MAX_BITS: usize = 9;
    const MAX_VALUE: u32 = 1 << MAX_BITS;

    // Fill the palette with values
    for i in 0..MAX_VALUE {
        let pos = ChunkBlockPos::from_index(i as usize);
        default.set(i, &pos);
    }

    // Check that the values are correct
    for i in 0..MAX_VALUE {
        let pos = ChunkBlockPos::from_index(i as usize);
        assert_eq!(default.get(&pos), i);
    }

    // Check that the palette expanded correctly
    assert_eq!(default.palette, Palette::Global);
    assert_eq!(default.bits, MAX_BITS);
}

#[test]
fn wiki_example() {
    let container = Container::<BlockContainer> {
        bits: 5,
        palette: Palette::Vector(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ]),
        data: BitVec::from_slice(&[
            0b0000000000100000100001100011000101001000010000011000100001000001,
            0b0000000100000001100010100111001001100000111101101000110010000111,
        ]),
        _phantom: std::marker::PhantomData,
    };

    for (i, n) in [
        1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 2u32,
    ]
    .into_iter()
    .enumerate()
    {
        let pos = ChunkBlockPos::from_index(i);
        assert_eq!(container.get(&pos), n);
    }
}
