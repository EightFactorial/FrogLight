use std::{io::Cursor, marker::PhantomData};

use bevy_reflect::Reflect;
use bitvec::prelude::{BitVec, Msb0};
use froglight_protocol::io::FrogRead;

use crate::world::{chunk::ChunkDecodeError, Palette};

/// A `ChunkDataContainer` is used to store a type of data in a
/// [`Section`](crate::world::Section).
///
/// A [`BlockContainer`] contains block data.
///
/// A [`BiomeContainer`] contains biome data.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct ChunkDataContainer<T: ContainerType> {
    /// The number of bits used to store each entry in the container.
    pub bits: usize,
    /// The palette type used by the container.
    pub palette: Palette,
    /// The data stored in the container.
    #[reflect(ignore)]
    pub data: BitVec<u64, Msb0>,
    #[reflect(ignore)]
    pub(crate) _phantom: PhantomData<T>,
}

impl<T: ContainerType> ChunkDataContainer<T> {
    /// Decodes a [`Container`] from a buffer.
    pub(crate) fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, ChunkDecodeError> {
        // Read the bit count
        let bits = usize::from(u8::fg_read(buf)?);

        // Decode the palette
        let mut palette = T::palette_type(bits);
        palette = palette.decode(buf)?;

        let data = BitVec::try_from_vec(Vec::<u64>::fg_read(buf)?)
            .map_err(|_| ChunkDecodeError::BitVec)?;

        Ok(Self { bits, palette, data, _phantom: PhantomData })
    }

    /// Creates a new [`ChunkDataContainer`] with the given data.
    #[must_use]
    #[inline]
    pub fn new(bits: usize, palette: Palette, data: BitVec<u64, Msb0>) -> Self {
        Self { bits, palette, data, _phantom: PhantomData }
    }
}

/// A [`ContainerType`] is a kind of data that can be stored in a
/// [`ChunkDataContainer`].
pub trait ContainerType: Reflect {
    /// Returns the palette type for a given number of bits.
    #[must_use]
    fn palette_type(bits: usize) -> Palette;
}

/// A [`ChunkDataContainer`] that stores block data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockContainer;

impl ContainerType for BlockContainer {
    fn palette_type(bits: usize) -> Palette {
        match bits {
            0 => Palette::Single(0u32),
            1..=8 => Palette::Vector(Vec::new()),
            _ => Palette::Global,
        }
    }
}

/// A [`ChunkDataContainer`] that stores biome data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BiomeContainer;

impl ContainerType for BiomeContainer {
    fn palette_type(bits: usize) -> Palette {
        match bits {
            0 => Palette::Single(0u32),
            1..=3 => Palette::Vector(Vec::new()),
            _ => Palette::Global,
        }
    }
}

// #[cfg(test)]
// proptest::proptest! {
//     #[test]
//     fn block_container_test(coordinates in
// proptest::collection::vec(proptest::array::uniform3(0u8..16u8), 0..4096)) {
//         let positions: Vec<_> = coordinates.iter().map(|[x, y, z]|
// froglight_core::common::SectionBlockPosition::new(*x, *y, *z)).collect();

//         let mut container = ChunkDataContainer::<BlockContainer>::default();
//         for (index, position) in positions.iter().enumerate() {
//             container.set_data(position, index);
//         }
//         for (index, position) in positions.iter().enumerate() {
//             assert_eq!(container.get_data(position), index);
//         }
//     }
// }
