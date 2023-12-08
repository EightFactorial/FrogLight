use std::io::Cursor;

use super::{palette::Palette, section::Section, tasks::ChunkDecodeError};

pub mod traits;
use bevy::log::error;
use bitvec::{order::Msb0, vec::BitVec};
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
    pub bits: u8,
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
    pub fn get_data(&self, pos: &ChunkBlockPos) -> Option<u32> {
        // If the bitsize is 0, the Palette is a single value
        if self.bits == 0 {
            if let Palette::Single(id) = &self.palette {
                return Some(*id);
            } else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!("Palette has a bitsize of 0, but is not Single!");
                return None;
            }
        }

        // Get the index of the block in the chunk
        let block_index = (pos.x as usize % Section::SECTION_WIDTH)
            + ((pos.z as usize % Section::SECTION_DEPTH) * Section::SECTION_WIDTH)
            + ((pos.y as usize % Section::SECTION_HEIGHT)
                * Section::SECTION_WIDTH
                * Section::SECTION_DEPTH);

        // Get the data index and length
        let data_index = block_index * self.bits as usize;

        // Get the data slice
        let Some(slice) = self.data.get(data_index..data_index + self.bits as usize) else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to get block at {pos:?} from Container (b:{}, l:{})",
                self.bits,
                self.data.len()
            );
            return None;
        };

        // Create the palette index
        let mut palette_index = 0u32;
        for (i, bit) in slice.iter().rev().enumerate() {
            palette_index += if *bit { 1 << i } else { 0 };
        }

        // If the palette is Global, return the palette index
        if self.palette == Palette::Global {
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

        val
    }

    /// Set the data at the given position.
    pub fn set_data(&mut self, _data: u32, _pos: &ChunkBlockPos) { todo!() }
}

#[test]
fn get_block() {
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
