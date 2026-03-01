use alloc::vec::Vec;

use bitvec::prelude::BitVec;
use smallvec::SmallVec;

use crate::{
    SECTION_HEIGHT,
    chunk::{
        Section,
        section::{SectionData, SectionPalette},
    },
    prelude::*,
    section::{BiomeSection, BlockSection, SectionPaletteType, SectionType},
};

/// An error that can occur when parsing chunk data.
#[derive(Debug)]
pub enum ParseError {
    /// The input data was not long enough
    EndOfInput,
}

impl NaiveChunk {
    /// Attempt to parse a [`NaiveChunk`] from the given data.
    ///
    /// Inputs:
    ///   - `height_max`: The maximum Y-level in the chunk.
    ///   - `height_min`: The minimum Y-level in the chunk.
    ///
    /// # Errors
    ///
    /// Returns an error of the input is not a valid chunk.
    pub fn try_from(
        input: &[u8],
        height_max: u32,
        height_min: i32,
    ) -> Result<NaiveChunk, ParseError> {
        Self::try_from_remainder(input, height_max, height_min).map(|(chunk, _)| chunk)
    }

    /// Attempt to parse a [`NaiveChunk`] from the given data,
    /// returning any remaining data left over.
    ///
    /// Inputs:
    ///   - `height_max`: The maximum Y-level in the chunk.
    ///   - `height_min`: The minimum Y-level in the chunk.
    ///
    /// # Errors
    ///
    /// Returns an error of the input is not a valid chunk.
    pub fn try_from_remainder(
        mut input: &[u8],
        height_max: u32,
        height_min: i32,
    ) -> Result<(NaiveChunk, &[u8]), ParseError> {
        let Some(total_height) = height_max.checked_sub_signed(height_min) else { todo!() };
        let section_count = total_height / u32::from(SECTION_HEIGHT);

        let mut sections = Vec::with_capacity(section_count as usize);
        for _ in 0..section_count {
            let (section, remainder) = Section::try_from_remainder(input)?;
            sections.push(section);
            input = remainder;
        }

        Ok((NaiveChunk::new_from(sections, height_min), input))
    }
}

impl Section {
    /// Attempt to parse a [`Section`] from the given data.
    ///
    /// # Errors
    ///
    /// Returns an error of the input is not a valid section.
    pub fn try_from(input: &[u8]) -> Result<Section, ParseError> {
        Self::try_from_remainder(input).map(|(section, _)| section)
    }

    /// Attempt to parse a [`Section`] from the given data,
    /// returning any remaining data left over.
    ///
    /// # Errors
    ///
    /// Returns an error of the input is not a valid section.
    pub fn try_from_remainder(input: &[u8]) -> Result<(Section, &[u8]), ParseError> {
        let Some((block_count, input)) = input.split_first_chunk() else { todo!() };
        let block_count = u16::from_be_bytes(*block_count);

        let Some((fluid_count, input)) = input.split_first_chunk() else { todo!() };
        let fluid_count = u16::from_be_bytes(*fluid_count);

        let (blocks, input) = SectionData::<BlockSection>::try_from_remainder(input)?;
        let (biomes, input) = SectionData::<BiomeSection>::try_from_remainder(input)?;

        // SAFETY: Input was parsed and is valid
        unsafe { Ok((Section::new_unchecked(block_count, fluid_count, blocks, biomes), input)) }
    }
}

impl<T: SectionType> SectionData<T> {
    /// Attempt to parse a [`SectionData`] from the given data.
    ///
    /// # Errors
    ///
    /// Returns an error of the input is not a valid section data.
    pub fn try_from(input: &[u8]) -> Result<SectionData<T>, ParseError> {
        Self::try_from_remainder(input).map(|(section, _)| section)
    }

    /// Attempt to parse a [`SectionData`] from the given data,
    /// returning any remaining data left over.
    ///
    /// # Errors
    ///
    /// Returns an error of the input is not a valid section data.
    pub fn try_from_remainder(input: &[u8]) -> Result<(SectionData<T>, &[u8]), ParseError> {
        let Some((&bits, mut input)) = input.split_first() else { todo!() };

        let palette = match T::palette_for(bits) {
            SectionPaletteType::Single => {
                let Some((value, remaining)) = bytes_to_variable(input) else { todo!() };
                input = remaining;

                SectionPalette::Single(value)
            }
            SectionPaletteType::Vector => {
                let Some((length, remaining)) = bytes_to_variable(input) else { todo!() };
                input = remaining;

                let mut values = SmallVec::with_capacity(length as usize);
                for _ in 0..length {
                    let Some((value, remaining)) = bytes_to_variable(input) else { todo!() };
                    input = remaining;
                    values.push(value);
                }

                SectionPalette::Vector(values)
            }
            SectionPaletteType::Global => SectionPalette::Global,
        };

        let length =
            if bits == 0 { 0 } else { u32::from(T::VOLUME).div_ceil(u64::BITS / u32::from(bits)) };
        let mut vec = Vec::<u64>::with_capacity(length as usize);
        for _ in 0..length {
            let Some((val, remainder)) = input.split_first_chunk() else { todo!() };
            vec.push(u64::from_ne_bytes(*val));
            input = remainder;
        }

        // SAFETY: Input was parsed and is valid
        unsafe {
            Ok((Self::new_unchecked(usize::from(bits), palette, BitVec::from_vec(vec)), input))
        }
    }
}

/// Read a variable-length integer from a buffer.
///
/// Returns the value and the remaining buffer.
fn bytes_to_variable(bytes: &[u8]) -> Option<(u32, &[u8])> {
    let mut byte: u8;
    let mut index: usize = 0;
    let mut number: u32 = 0;

    while index < 5 {
        byte = *bytes.get(index)?;
        number |= u32::from(byte & 0b0111_1111) << (7 * index);
        index += 1;
        if byte & 0b1000_0000 == 0 {
            break;
        }
    }

    Some((number, &bytes[index..]))
}
