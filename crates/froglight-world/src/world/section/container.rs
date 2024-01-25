use std::io::Cursor;

use bitvec::prelude::{BitVec, Msb0};
use froglight_protocol::io::FrogRead;

use super::Palette;
use crate::world::chunk::ChunkDecodeError;

/// A [`Container`] is used to store a type of data in a [`Section`].
///
/// For [`BlockContainer`], it contains block data.
///
/// For [`BiomeContainer`], it contains biome data.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Container<T: ContainerType> {
    /// The number of bits used to store each entry in the container.
    pub bits: usize,
    /// The palette type used by the container.
    pub palette: Palette,
    /// The data stored in the container.
    pub data: BitVec<u64, Msb0>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: ContainerType> Container<T> {
    /// Decodes a [`Container`] from a buffer.
    pub(crate) fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, ChunkDecodeError> {
        let bits = usize::from(u8::frog_read(buf)?);

        let mut palette = T::palette_type(bits);
        palette = palette.decode(buf)?;

        let data = BitVec::try_from_vec(Vec::<u64>::frog_read(buf)?)
            .map_err(|_| ChunkDecodeError::BitVec)?;

        Ok(Self { bits, palette, data, _phantom: std::marker::PhantomData })
    }
}

/// A [`ContainerType`] is a kind of data that can be stored in a [`Container`].
pub trait ContainerType {
    /// Returns the palette type for a given number of bits.
    fn palette_type(bits: usize) -> Palette;
}

/// A [`Container`] that stores block data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

/// A [`Container`] that stores biome data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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
