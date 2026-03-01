//! TODO

use core::fmt::Debug;

use crate::SECTION_VOLUME;

/// A type of section storage.
pub trait SectionType: Debug + Default + Clone + Send + Sync + Sealed + 'static {
    /// The volume of this type of section.
    #[expect(clippy::cast_possible_truncation, reason = "Sections will never be that large")]
    const VOLUME: u16 =
        SECTION_VOLUME / (Self::QUANTIZATION * Self::QUANTIZATION * Self::QUANTIZATION) as u16;
    /// The quantization factor of this type of section.
    const QUANTIZATION: usize;

    /// Get a [`SectionPaletteType`] for this number of bits.
    fn palette_for(bits: u8) -> SectionPaletteType;
}

/// A type of section palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionPaletteType {
    /// A single value.
    Single,
    /// A list of values that can be indexed into.
    Vector,
    /// Values should be used directly.
    Global,
}

use sealed::Sealed;
mod sealed {
    pub trait Sealed {}
}

// -------------------------------------------------------------------------------------------------

/// A storage container for block data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSection;

impl Sealed for BlockSection {}
impl SectionType for BlockSection {
    const QUANTIZATION: usize = 1;

    fn palette_for(bits: u8) -> SectionPaletteType {
        match bits {
            0 => SectionPaletteType::Single,
            1..=8 => SectionPaletteType::Vector,
            _ => SectionPaletteType::Global,
        }
    }
}

/// A storage container for biome data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BiomeSection;

impl Sealed for BiomeSection {}
impl SectionType for BiomeSection {
    const QUANTIZATION: usize = 4;

    fn palette_for(bits: u8) -> SectionPaletteType {
        match bits {
            0 => SectionPaletteType::Single,
            1..=3 => SectionPaletteType::Vector,
            _ => SectionPaletteType::Global,
        }
    }
}
