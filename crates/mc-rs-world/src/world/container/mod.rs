use std::{cmp::Ordering, io::Cursor};

use super::{palette::Palette, section::Section, tasks::ChunkDecodeError};

pub mod traits;
use bevy::log::error;
use bitvec::{order::Msb0, slice::BitSlice, vec::BitVec};
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
    /// Get the data at the given position in the [`Container`].
    ///
    /// This assumes that the position is within the [`Section`],
    /// and any vertical shifting has already been applied.
    pub fn get_data(&self, pos: &ChunkBlockPos) -> Option<u32> {
        // If the palette is Single, return the palette id
        if let Palette::Single(id) = &self.palette {
            return Some(*id);
        }

        // Get the data slice
        let data_index = self.data_index_from_pos(pos);
        let slice = &self.data[data_index..data_index + self.bits];

        // Create the palette index
        let mut palette_index = 0u32;
        for (i, bit) in slice.iter().rev().enumerate() {
            palette_index += if *bit { 1 << i } else { 0 };
        }

        // If the palette is Global, return the palette index
        if let Palette::Global = &self.palette {
            return Some(palette_index);
        }

        // Convert the palette index to a usize
        let Ok(palette_index) = usize::try_from(palette_index) else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Palette Index `{palette_index}` overflowed!");
            return None;
        };

        // Get the palette value
        let val = self.palette.get(palette_index);

        // Debug logging
        #[cfg(any(debug_assertions, feature = "debug"))]
        if val.is_none() {
            error!(
                "Failed to get Index `{palette_index}` from Palette ({:?})",
                self.palette
            );
        }

        val.copied()
    }

    /// Set the data at the given position.
    pub fn set_data(&mut self, data: u32, pos: &ChunkBlockPos) {
        match &mut self.palette {
            Palette::Single(id) => {
                // If the data is the same as the palette id, do nothing
                if data != *id {
                    // Change the palette to Vector
                    self.palette = Palette::Vector(vec![*id, data]);
                    // Set the bits to 1
                    self.bits = 1;
                    // Fill the Container with zeros
                    self.data = BitVec::repeat(false, Section::SECTION_VOLUME);
                    // Insert the data
                    self.insert(1, self.data_index_from_pos(pos));
                }
            }
            Palette::Vector(ids) => {
                if let Some(index) = ids.iter().position(|id| id == &data) {
                    // Insert the data
                    self.insert(index, self.data_index_from_pos(pos));

                    // TODO: Shrink the Container if needed
                    // Note: Requires re-encoding the Container...
                } else {
                    // Get the index of the new data
                    let index = ids.len();

                    // Add the data to the palette
                    let mut expanded_ids = ids.clone();
                    expanded_ids.push(data);
                    *ids = expanded_ids;

                    // TODO: Check if the palette needs to be converted to Global
                    // Expand the Container if needed
                    if (ids.len() - 1) >= 2usize.pow(self.bits as u32) {
                        self.expand(1);
                    }

                    // Insert the data
                    self.insert(index, self.data_index_from_pos(pos));
                }
            }
            Palette::Global => {
                let data_bits = BitSlice::<u32, Msb0>::from_element(&data);
                let first_one = data_bits.iter().position(|bit| *bit).unwrap_or(0);

                // Expand or shrink the Container if needed
                match first_one.cmp(&self.bits) {
                    Ordering::Greater => {
                        // Expand the Container
                        self.expand(first_one - self.bits);

                        // Insert the data
                        self.insert(
                            data.try_into().expect("Container data overflow"),
                            self.data_index_from_pos(pos),
                        );
                    }
                    Ordering::Equal => {
                        // Insert the data
                        self.insert(
                            data.try_into().expect("Container data overflow"),
                            self.data_index_from_pos(pos),
                        );
                    }
                    Ordering::Less => {
                        // Insert the data
                        self.insert(
                            data.try_into().expect("Container data overflow"),
                            self.data_index_from_pos(pos),
                        );

                        // TODO: Shrink the Container if needed
                        // Note: Requires knowing the largest number in the Container...
                    }
                }
            }
        }
    }

    /// Expand the [`Container`]'s bitsize by the given amount.
    ///
    /// This pads every index with `count` zeros.
    fn expand(&mut self, count: usize) {
        let new_bits = self.bits + count;

        // Create a new expanded Container
        let mut expanded = Self {
            bits: new_bits,
            palette: self.palette.clone(),
            data: BitVec::repeat(false, Section::SECTION_VOLUME * new_bits),
            _phantom: std::marker::PhantomData,
        };

        // Copy the data from the old Container to the new Container
        let mut index = 0usize;
        for bit in self.data.as_bitslice() {
            // Pad the index with zeros
            if self.bits - (index % new_bits) >= count {
                expanded.data.set(index, false);
                index += 1;
            }
            expanded.data.set(index, *bit);
            index += 1;
        }

        // Replace the old Container with the new Container
        *self = expanded;
    }

    /// Insert the palette index into the data bits.
    fn insert(&mut self, index: usize, data_index: usize) {
        // Get the data bits
        let slice = &mut self.data[data_index..data_index + self.bits];

        // Get the input bits
        let input_slice = BitSlice::<usize, Msb0>::from_element(&index);
        let input_slice = &input_slice[usize::BITS as usize - self.bits..];

        // Copy the input bits to the data bits
        for (i, bit) in input_slice.into_iter().enumerate() {
            slice.set(i, *bit);
        }
    }

    /// Convert a [`ChunkBlockPos`] to a data index.
    ///
    /// # Warning
    /// This index is invalid if the palette is modified after the index is created.
    fn data_index_from_pos(&self, pos: &ChunkBlockPos) -> usize {
        // Get the index of the block in the chunk
        let block_index = (pos.x as usize % Section::SECTION_WIDTH)
            + ((pos.z as usize % Section::SECTION_DEPTH) * Section::SECTION_WIDTH)
            + ((pos.y as usize % Section::SECTION_HEIGHT)
                * Section::SECTION_WIDTH
                * Section::SECTION_DEPTH);

        // Get the data index and length
        block_index * self.bits
    }
}

#[test]
fn get_data() {
    // Empty container, should return 0
    let container = Container::<BlockContainer>::default();
    assert_eq!(container.get_data(&ChunkBlockPos::new(0, 0, 0)), Some(0));
    assert_eq!(container.get_data(&ChunkBlockPos::new(8, 8, 8)), Some(0));
    assert_eq!(container.get_data(&ChunkBlockPos::new(16, 16, 16)), Some(0));

    // Two possible blocks, should return repeating 0 and 1
    let container = Container::<BlockContainer> {
        bits: 1,
        palette: Palette::Vector(vec![0, 1]),
        data: BitVec::from_slice(&[
            0b0101010101010101010101010101010101010101010101010101010101010101u64,
            0b0101010101010101010101010101010101010101010101010101010101010101u64,
            0b0101010101010101010101010101010101010101010101010101010101010101u64,
            0b0101010101010101010101010101010101010101010101010101010101010101u64,
        ]),
        _phantom: std::marker::PhantomData,
    };

    for z in 0..16u8 {
        for x in 0..16u8 {
            let pos = ChunkBlockPos::new(x, 0, z);
            assert_eq!(container.get_data(&pos), Some(u32::from(x % 2)));
        }
    }

    // Four possible blocks, should return repeating 0, 1, 2, 3
    let container = Container::<BlockContainer> {
        bits: 2,
        palette: Palette::Vector(vec![0, 1, 2, 3]),
        data: BitVec::from_slice(&[
            0b0001101100011011000110110001101100011011000110110001101100011011u64,
            0b0001101100011011000110110001101100011011000110110001101100011011u64,
            0b0001101100011011000110110001101100011011000110110001101100011011u64,
            0b0001101100011011000110110001101100011011000110110001101100011011u64,
            0b0001101100011011000110110001101100011011000110110001101100011011u64,
            0b0001101100011011000110110001101100011011000110110001101100011011u64,
            0b0001101100011011000110110001101100011011000110110001101100011011u64,
            0b0001101100011011000110110001101100011011000110110001101100011011u64,
        ]),
        _phantom: std::marker::PhantomData,
    };

    for z in 0..16u8 {
        for x in 0..16u8 {
            let pos = ChunkBlockPos::new(x, 0, z);
            assert_eq!(container.get_data(&pos), Some(u32::from(x % 4)));
        }
    }

    // Get select blocks from a container with 2 possible blocks
    let container = Container::<BlockContainer> {
        bits: 1,
        palette: Palette::Vector(vec![0, 8]),
        data: BitVec::from_slice(&[
            0b1100000000000000000000000000000000000000000000000000000000000001u64,
            0b1000000000000000000000000000000000000000000000000000000000000000u64,
            0b0000000000000000000000000000011000000000000000000000000000000000u64,
            0b0000000010000000000000000000000001000000000000000000000010000000u64,
        ]),
        _phantom: std::marker::PhantomData,
    };

    // First row
    assert_eq!(container.get_data(&ChunkBlockPos::from_index(0)), Some(8));
    assert_eq!(container.get_data(&ChunkBlockPos::from_index(1)), Some(8));

    // Second row
    assert_eq!(
        container.get_data(&ChunkBlockPos::from_index(64 - 1)),
        Some(8)
    );
    assert_eq!(container.get_data(&ChunkBlockPos::from_index(64)), Some(8));

    // Third row
    assert_eq!(
        container.get_data(&ChunkBlockPos::from_index(128 + 29)),
        Some(8)
    );
    assert_eq!(
        container.get_data(&ChunkBlockPos::from_index(128 + 30)),
        Some(8)
    );

    // Fourth row
    assert_eq!(
        container.get_data(&ChunkBlockPos::from_index(192 + 8)),
        Some(8)
    );
    assert_eq!(
        container.get_data(&ChunkBlockPos::from_index(192 + 33)),
        Some(8)
    );
    assert_eq!(
        container.get_data(&ChunkBlockPos::from_index(192 + 56)),
        Some(8)
    );
}

#[test]
fn add_data() {
    let mut default = Container::<BlockContainer>::default();
    let first = ChunkBlockPos::new(0, 0, 0);
    let second = ChunkBlockPos::new(1, 0, 0);
    let third = ChunkBlockPos::new(2, 0, 0);

    assert_eq!(default.bits, 0);
    assert_eq!(default.palette, Palette::Single(0));
    assert_eq!(default.data.len(), 0);
    assert_eq!(default.get_data(&first), Some(0));
    assert_eq!(default.get_data(&second), Some(0));
    assert_eq!(default.get_data(&third), Some(0));

    // Add a block to an empty container
    default.set_data(1, &first);

    assert_eq!(default.bits, 1);
    assert_eq!(default.palette, Palette::Vector(vec![0, 1]));
    assert_eq!(default.get_data(&first), Some(1));
    assert_eq!(default.get_data(&second), Some(0));
    assert_eq!(default.get_data(&third), Some(0));

    // Add a second block to the container
    default.set_data(8, &second);

    assert_eq!(default.bits, 2);
    assert_eq!(default.palette, Palette::Vector(vec![0, 1, 8]));
    assert_eq!(default.get_data(&first), Some(1));
    assert_eq!(default.get_data(&second), Some(8));
    assert_eq!(default.get_data(&third), Some(0));

    // Add a third block to the container
    default.set_data(2, &third);

    assert_eq!(default.bits, 2);
    assert_eq!(default.palette, Palette::Vector(vec![0, 1, 8, 2]));
    assert_eq!(default.get_data(&first), Some(1));
    assert_eq!(default.get_data(&second), Some(8));
    assert_eq!(default.get_data(&third), Some(2));

    // Remove the second and third blocks from the container
    default.set_data(0, &second);
    default.set_data(0, &third);

    // SHRINKING IS NOT IMPLEMENTED YET
    assert_eq!(default.bits, 2);
    assert_eq!(default.palette, Palette::Vector(vec![0, 1, 8, 2]));
    assert_eq!(default.get_data(&first), Some(1));
    assert_eq!(default.get_data(&second), Some(0));
    assert_eq!(default.get_data(&third), Some(0));
}
