//! TODO
#![allow(dead_code, reason = "WIP")]

use core::{marker::PhantomData, ops::Range};

use bitvec::{field::BitField, order::Msb0, slice::BitSlice};

use crate::{
    SECTION_HEIGHT, SECTION_WIDTH,
    component::SectionBlockPos,
    section::{BiomeSection, BlockSection, SectionType},
};

/// A borrowed piece of a chunk.
#[derive(Default, Clone)]
pub struct BorrowedSection<'a> {
    non_air: u32,
    blocks: BorrowedSectionData<'a, BlockSection>,
    biomes: BorrowedSectionData<'a, BiomeSection>,
}

impl<'a> BorrowedSection<'a> {
    /// Create a new [`BorrowedSection`] without performing any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided data is valid.
    #[must_use]
    pub const unsafe fn new_unchecked(
        non_air: u32,
        blocks: BorrowedSectionData<'a, BlockSection>,
        biomes: BorrowedSectionData<'a, BiomeSection>,
    ) -> Self {
        Self { non_air, blocks, biomes }
    }

    /// Get the number of non-air blocks in this section.
    #[must_use]
    pub const fn block_count(&self) -> u32 { self.non_air }

    /// Get the [`BorrowedSectionData`] for blocks.
    #[must_use]
    pub const fn block_data(&self) -> &BorrowedSectionData<'a, BlockSection> { &self.blocks }

    /// Get the [`BorrowedSectionData`] for biomes.
    #[must_use]
    pub const fn biome_data(&self) -> &BorrowedSectionData<'a, BiomeSection> { &self.biomes }

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
pub struct BorrowedSectionData<'a, T: SectionType> {
    bits: usize,
    palette: BorrowedPalette<'a>,
    data: &'a BitSlice<u64, Msb0>,
    _phantom: PhantomData<T>,
}

impl<'a, T: SectionType> BorrowedSectionData<'a, T> {
    /// Create a new [`BorrowedSectionData`] without performing any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided data is valid.
    #[must_use]
    pub const unsafe fn new_unchecked(
        bits: usize,
        palette: BorrowedPalette<'a>,
        data: &'a BitSlice<u64, Msb0>,
    ) -> Self {
        Self { bits, palette, data, _phantom: PhantomData }
    }

    /// Get the number of bits per entry.
    #[must_use]
    pub const fn bits_per_entry(&self) -> usize { self.bits }

    /// Get the palette used by this data.
    #[must_use]
    pub const fn palette(&self) -> &BorrowedPalette<'a> { &self.palette }

    /// Get the raw bit data.
    #[must_use]
    pub const fn data(&self) -> &'a BitSlice<u64, Msb0> { self.data }

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

        if let BorrowedPalette::Single(value) = self.palette {
            return Some(value);
        }

        let index = self.get_slice(index)?.load::<usize>();
        match self.palette {
            BorrowedPalette::Single(_) => unreachable!(),
            BorrowedPalette::Vector(items) => items.get(index).copied(),
            BorrowedPalette::Global => index.try_into().ok(),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowedPalette<'a> {
    /// A single value.
    Single(u32),
    /// A list of values that can be indexed into.
    Vector(&'a [u32]),
    /// Values should be used directly.
    Global,
}

impl Default for BorrowedPalette<'_> {
    fn default() -> Self { Self::Single(0u32) }
}
