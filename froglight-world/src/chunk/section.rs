//! TODO

use core::{marker::PhantomData, ops::Range};

use bitvec::{field::BitField, order::Msb0, slice::BitSlice, vec::BitVec};
use smallvec::SmallVec;

use crate::{
    SECTION_HEIGHT, SECTION_WIDTH,
    component::SectionBlockPos,
    section::{BiomeSection, BlockSection, SectionType},
};

/// A piece of a chunk.
#[derive(Default, Clone)]
pub struct Section {
    non_air: u32,
    blocks: SectionData<BlockSection>,
    biomes: SectionData<BiomeSection>,
}

impl Section {
    /// Create a new [`BorrowedSection`] without performing any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided data is valid.
    #[must_use]
    pub const unsafe fn new_unchecked(
        non_air: u32,
        blocks: SectionData<BlockSection>,
        biomes: SectionData<BiomeSection>,
    ) -> Self {
        Self { non_air, blocks, biomes }
    }

    /// Get the number of non-air blocks in this section.
    #[must_use]
    pub const fn block_count(&self) -> u32 { self.non_air }

    /// Get the [`SectionData`] for blocks.
    #[must_use]
    pub const fn block_data(&self) -> &SectionData<BlockSection> { &self.blocks }

    /// Get the [`SectionData`] for blocks mutably.
    #[must_use]
    pub const fn block_data_mut(&mut self) -> &mut SectionData<BlockSection> { &mut self.blocks }

    /// Get the [`SectionData`] for biomes.
    #[must_use]
    pub const fn biome_data(&self) -> &SectionData<BiomeSection> { &self.biomes }

    /// Get the [`SectionData`] for biomes mutably.
    #[must_use]
    pub const fn biome_data_mut(&mut self) -> &mut SectionData<BiomeSection> { &mut self.biomes }

    /// Get the block id at the given position within the section.
    #[inline]
    #[must_use]
    pub fn get_raw_block(&self, position: SectionBlockPos) -> u32 { self.blocks.get(position) }

    /// Get the biome id at the given position within the section.
    #[inline]
    #[must_use]
    pub fn get_raw_biome(&self, position: SectionBlockPos) -> u32 { self.biomes.get(position) }
}

// -------------------------------------------------------------------------------------------------

/// A bit-packed bundle of chunk data.
#[derive(Default, Clone)]
pub struct SectionData<T: SectionType> {
    bits: usize,
    palette: SectionPalette,
    data: BitVec<u64, Msb0>,
    _phantom: PhantomData<T>,
}

impl<T: SectionType> SectionData<T> {
    /// Create a new [`BorrowedSectionData`] without performing any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided data is valid.
    #[must_use]
    pub const unsafe fn new_unchecked(
        bits: usize,
        palette: SectionPalette,
        data: BitVec<u64, Msb0>,
    ) -> Self {
        Self { bits, palette, data, _phantom: PhantomData }
    }

    /// Get the number of bits per entry.
    #[must_use]
    pub const fn bits_per_entry(&self) -> usize { self.bits }

    /// Get the palette used by this data.
    #[must_use]
    pub const fn palette(&self) -> &SectionPalette { &self.palette }

    /// Get the raw bit data.
    #[must_use]
    pub fn data(&self) -> &BitSlice<u64, Msb0> { self.data.as_bitslice() }

    /// Get the value at the given position within the section.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "The index cannot ever go out of bounds")]
    pub fn get(&self, position: SectionBlockPos) -> u32 {
        let width = usize::from(SECTION_WIDTH) / T::QUANTIZATION;
        let height = usize::from(SECTION_HEIGHT) / T::QUANTIZATION;

        self.get_index(
            (usize::from(position.x()) / T::QUANTIZATION)
                + (usize::from(position.z()) / T::QUANTIZATION * width)
                + (usize::from(position.y()) / T::QUANTIZATION * width * height),
        )
        .expect("SectionBlockPos should always be within bounds")
    }

    /// Get the value at the given index within the section.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<u32> {
        if index > usize::from(T::VOLUME) {
            return None;
        }

        if let SectionPalette::Single(value) = self.palette {
            return Some(value);
        }

        let index = self.get_slice(index)?.load::<usize>();
        match &self.palette {
            SectionPalette::Single(_) => unreachable!(),
            SectionPalette::Vector(items) => items.get(index).copied(),
            SectionPalette::Global => index.try_into().ok(),
        }
    }

    /// Get a reference to the entry at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get_slice(&self, index: usize) -> Option<&BitSlice<u64, Msb0>> {
        let start = index * self.bits;
        self.data.get(Range { start, end: start + self.bits })
    }
}

// -------------------------------------------------------------------------------------------------

/// A borrowed palette of values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SectionPalette {
    /// A single value.
    Single(u32),
    /// A list of values that can be indexed into.
    Vector(SmallVec<[u32; 8]>),
    /// Values should be used directly.
    Global,
}

impl Default for SectionPalette {
    fn default() -> Self { Self::Single(0u32) }
}
