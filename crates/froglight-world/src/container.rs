use std::marker::PhantomData;

use bitvec::{
    field::BitField,
    prelude::{BitVec, Msb0},
    slice::BitSlice,
};
use froglight_protocol::{
    common::SectionBlockPosition,
    protocol::{FrogRead, FrogVarWrite, FrogWrite, ReadError, WriteError},
};

use self::sealed::ContainerType;
use crate::{palette::ContainerPalette, ChunkSection};

/// A container for storing data.
#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct Container<T: ContainerType> {
    /// The number of bits used to store each entry in the container.
    pub bits: usize,
    /// The palette type used by the container.
    pub palette: ContainerPalette,
    /// The data stored in the container.
    pub data: BitVec<u64, Msb0>,
    pub(crate) _phantom: PhantomData<T>,
}

impl<T: ContainerType> Container<T> {
    /// Gets the value at the given coordinates.
    ///
    /// # Panics
    /// Panics if the palette value is out of range.
    #[must_use]
    pub fn get_data(&self, pos: &SectionBlockPosition) -> u32 {
        // Skip the lookup if the palette only contains a single value.
        if let ContainerPalette::Single(v) = self.palette {
            return v;
        }

        // Load the value from the bitslice and convert it to a usize.
        let slice = self.get_bitslice(*pos);
        let value = slice.load_be::<u32>();

        match &self.palette {
            // Get the value from the palette if it's a vector.
            ContainerPalette::Vector(vec) => {
                if let Some(value) = vec.get(value as usize) {
                    *value
                } else {
                    #[cfg(feature = "bevy")]
                    bevy_log::error!("Value in BitVec does not exist in ContainerPalette::Vector!");
                    0
                }
            }
            // Return the value directly if the palette is global.
            ContainerPalette::Global => value,
            ContainerPalette::Single(_) => {
                unreachable!("ContainerPalette::Single was handled earlier")
            }
        }
    }

    /// Sets the value at the given coordinates.
    ///
    /// Returns the previous value.
    #[allow(clippy::missing_panics_doc)]
    pub fn set_data(&mut self, pos: &SectionBlockPosition, value: u32) -> u32 {
        match &self.palette {
            ContainerPalette::Single(_) => self.set_single(*pos, value),
            ContainerPalette::Vector(_) => self.set_vector(*pos, value),
            ContainerPalette::Global => self.set_global(*pos, value),
        }
    }

    /// Set a value inside a [`ContainerPalette::Single`].
    fn set_single(&mut self, pos: SectionBlockPosition, value: u32) -> u32 {
        let ContainerPalette::Single(v) = &self.palette else {
            unreachable!("ContainerPalette must be ContainerPalette::Single");
        };

        if *v == value {
            // Do nothing, the value is already set.
            value
        } else {
            // Store the old value to return later.
            let old_value = *v;

            // Convert the palette to a vector and add the new value.
            self.palette = ContainerPalette::Vector(vec![*v, value]);

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

    /// Set a value inside a [`Palette::Vector`].
    #[allow(clippy::manual_unwrap_or_default)]
    fn set_vector(&mut self, pos: SectionBlockPosition, value: u32) -> u32 {
        let ContainerPalette::Vector(vec) = &self.palette else {
            unreachable!("ContainerPalette must be ContainerPalette::Vector");
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
                #[cfg(feature = "bevy")]
                bevy_log::error!("Value in BitVec does not exist in ContainerPalette::Vector!");
                0
            }
        } else {
            // TODO: Borrow checker >:(
            let mut vec = vec.clone();

            // Get the number of bits needed to store palette indexes.
            let required_size = Self::vector_bits_required(vec.len());

            // Check if the palette needs to be expanded.
            match T::palette_type(required_size) {
                ContainerPalette::Vector(_) => {
                    // Expand the bitvec to fit the new value.
                    if required_size > self.bits {
                        self.expand_bitvec_by(required_size - self.bits);
                    }

                    // Add the value to the palette.
                    let new_index = vec.len();
                    vec.push(value);

                    // Set the new palette.
                    self.palette = ContainerPalette::Vector(vec.clone());

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
                        #[cfg(feature = "bevy")]
                        bevy_log::error!(
                            "Value in BitVec does not exist in ContainerPalette::Vector!"
                        );
                        0
                    }
                }
                ContainerPalette::Global => {
                    // Convert the palette to a global palette.
                    self.convert_to_global();

                    // Set the value in the global palette.
                    self.set_global(pos, value)
                }
                ContainerPalette::Single(_) => {
                    unreachable!(
                        "Cannot create a ContainerPalette::Single from a ContainerPalette::Vector"
                    )
                }
            }
        }
    }

    /// Set a value inside a [`Palette::Global`].
    fn set_global(&mut self, pos: SectionBlockPosition, value: u32) -> u32 {
        let ContainerPalette::Global = &self.palette else {
            unreachable!("ContainerPalette must be ContainerPalette::Global");
        };

        // Check if the palette needs to be expanded.
        let required_size = Self::global_bits_required(value);
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

    /// Converts the [`Palette`] from [`Palette::Vector`] to
    /// [`Palette::Global`].
    fn convert_to_global(&mut self) {
        let ContainerPalette::Vector(vec) = &self.palette else {
            unreachable!("Only Palette::Vector can be converted to Palette::Global");
        };

        // Get the maximum value in the palette.
        let max_value = vec.iter().max().copied().unwrap();
        let required_size = Self::global_bits_required(max_value);

        // Create a new empty bitvec.
        let mut new_data = BitVec::repeat(false, Self::data_size_bits(required_size));

        // Copy the old data into the new bitvec.
        for index in 0..ChunkSection::VOLUME {
            let pos = SectionBlockPosition::from_index(index as usize);

            // Get the original data
            let old_slice = self.get_bitslice(pos);
            let old_index = old_slice.load_be::<usize>();
            let old_value = vec[old_index];

            // Copy the old bitslice into the new bitslice.
            let new_slice = &mut new_data[Self::entry_range(required_size, pos)];
            new_slice.store_be(old_value);
        }

        // Update the bits and data.
        self.palette = ContainerPalette::Global;
        self.bits = required_size;
        self.data = new_data;
    }

    /// Expands the [`BitVec`] by the given number of bits.
    fn expand_bitvec_by(&mut self, bits: usize) {
        // Calculate the new size.
        let new_bits = self.bits + bits;

        // Create a new bitvec with the new larger size.
        let mut new_data = BitVec::repeat(false, Self::data_size_bits(new_bits));

        // Copy the old data into the new bitvec.
        for index in 0..ChunkSection::VOLUME {
            let pos = SectionBlockPosition::from_index(index as usize);

            // Get the original bitslice
            let old_slice = self.get_bitslice(pos);

            // Get the new bitslice and match the size.
            let new_slice = &mut new_data[Self::entry_range(new_bits, pos)];
            let new_slice = &mut new_slice[bits..];

            // Copy the old bitslice into the new bitslice.
            new_slice.copy_from_bitslice(old_slice);
        }

        // Update the bits and data.
        self.bits = new_bits;
        self.data = new_data;
    }

    /// Compresses the [`BitVec`] and [`ContainerPalette`] to use the smallest
    /// possible size while still maintaining the same data.
    ///
    /// Warning: This is an expensive operation and should be used sparingly.
    pub fn compress(&mut self) {
        match &self.palette {
            ContainerPalette::Single(_) => {
                // Do nothing, the bitvec is already as small as possible.
            }
            ContainerPalette::Vector(_) => {
                todo! {
                    "Check for empty ContainerPalette indexes and remove them.
                    If there is only one value, convert to ContainerPalette::Single.
                    Reduce the bitsize if possible"
                };
            }
            ContainerPalette::Global => {
                todo! {
                    "Find the largest value and get the bitsize,
                    Potentially compress back into a ContainerPalette::Vector or ContainerPalette::Single"
                };
            }
        }
    }
}

/// Bitslice and calculation methods for accessing
/// data in a [`ChunkDataContainer`].
impl<T: ContainerType> Container<T> {
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
    pub(crate) const fn entry_range(
        bits: usize,
        pos: SectionBlockPosition,
    ) -> std::ops::Range<usize> {
        let start = Self::entry_start(bits, pos);
        std::ops::Range { start, end: start + bits }
    }

    /// Returns the start position of the entry in bits.
    #[must_use]
    pub(crate) const fn entry_start(bits: usize, pos: SectionBlockPosition) -> usize {
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
    pub(crate) const fn entries_per_long(bits: usize) -> usize { Self::U64BITS / bits }

    /// Returns the number of bits required to store a section.
    #[must_use]
    #[inline]
    pub(crate) const fn data_size_bits(bits: usize) -> usize {
        Self::data_size_longs(bits) * Self::U64BITS
    }

    /// Returns the number of longs required to store a section.
    #[must_use]
    #[inline]
    pub(crate) const fn data_size_longs(bits: usize) -> usize {
        let volume = ChunkSection::VOLUME as usize;
        volume.div_ceil(Self::entries_per_long(bits))
    }

    /// Returns the number of bits required to store the given number of
    /// entries.
    ///
    /// # Panics
    /// Panics if the given length is 0.
    #[must_use]
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) const fn vector_bits_required(len: usize) -> usize {
        (len as u32).ilog2() as usize + 1
    }

    /// Returns the number of bits required to store the given maximum value.
    #[must_use]
    #[inline]
    pub(crate) const fn global_bits_required(max: u32) -> usize {
        (u32::BITS - max.leading_zeros()) as usize
    }
}

pub(super) mod sealed {
    pub trait ContainerType {
        /// Returns the palette type for a given number of bits.
        #[must_use]
        fn palette_type(bits: usize) -> crate::palette::ContainerPalette;
    }
}

/// A [`Container`] that stores block data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockStorage;

/// A [`Container`] that stores block data.
pub type BlockContainer = Container<BlockStorage>;

impl ContainerType for BlockStorage {
    fn palette_type(bits: usize) -> ContainerPalette {
        match bits {
            0 => ContainerPalette::Single(0u32),
            1..=8 => ContainerPalette::Vector(Vec::new()),
            _ => ContainerPalette::Global,
        }
    }
}

/// A [`Container`] that stores biome data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BiomeStorage;

/// A [`Container`] that stores biome data.
pub type BiomeContainer = Container<BiomeStorage>;

impl ContainerType for BiomeStorage {
    fn palette_type(bits: usize) -> ContainerPalette {
        match bits {
            0 => ContainerPalette::Single(0u32),
            1..=3 => ContainerPalette::Vector(Vec::new()),
            _ => ContainerPalette::Global,
        }
    }
}

impl<T: ContainerType> std::fmt::Debug for Container<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Container")
            .field("bits", &self.bits)
            .field("palette", &self.palette)
            .field("data_len", &self.data.len())
            .finish()
    }
}

impl<T: ContainerType> FrogRead for Container<T> {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        // Read the bit count
        let bits = usize::from(u8::fg_read(buf)?);

        // Read the palette
        let mut palette = T::palette_type(bits);
        palette = palette.read_type(buf)?;

        // Read the data
        let data = Vec::<u64>::fg_read(buf)?;
        let data = BitVec::from_vec(data);

        Ok(Self { bits, palette, data, _phantom: PhantomData })
    }
}

impl<T: ContainerType> FrogWrite for Container<T> {
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        // Write the bit count
        FrogWrite::fg_write(&u8::try_from(self.bits).expect("Bitsize overflow"), buf)?;

        // Write the palette
        self.palette.fg_write(buf)?;

        // Write the data
        //
        // TODO: Optimize this
        let slice = self.data.as_raw_slice();
        slice.len().fg_var_write(buf)?;
        for long in slice {
            long.fg_write(buf)?;
        }

        Ok(())
    }
}
