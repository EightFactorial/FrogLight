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
    block_count: u32,
    blocks: SectionData<BlockSection>,
    biomes: SectionData<BiomeSection>,
}

impl Section {
    /// Create a new [`Section`] without performing any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided data is valid.
    #[must_use]
    pub const unsafe fn new_unchecked(
        block_count: u32,
        blocks: SectionData<BlockSection>,
        biomes: SectionData<BiomeSection>,
    ) -> Self {
        Self { block_count, blocks, biomes }
    }

    /// Get the number of non-air blocks in this section.
    #[must_use]
    pub const fn block_count(&self) -> u32 { self.block_count }

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

    /// Set the block id at the given position within the section,
    /// returning the previous id.
    ///
    /// The provided closure must return `true` if the block id corresponds with
    /// some form of air.
    #[must_use]
    pub fn set_raw_block(
        &mut self,
        position: SectionBlockPos,
        block_id: u32,
        mut is_air: impl FnMut(u32) -> bool,
    ) -> u32 {
        let previous = self.blocks.set(position, block_id);
        match (is_air(previous), is_air(block_id)) {
            // Non-air to air, decrement block counter.
            (false, true) => self.block_count -= 1,
            // Air to non-air, increment block counter.
            (true, false) => self.block_count += 1,
            _ => {}
        }
        previous
    }

    /// Create an iterator over all raw block ids in this section.
    #[inline]
    pub fn iter_raw_blocks(&self) -> impl Iterator<Item = u32> + '_ { self.blocks.iter() }

    /// Returns `true` if the given block id is contained within this section.
    #[inline]
    #[must_use]
    pub fn contains_raw_block(&self, id: u32) -> bool { self.blocks.contains(id) }

    /// Get the biome id at the given position within the section.
    #[inline]
    #[must_use]
    pub fn get_raw_biome(&self, position: SectionBlockPos) -> u32 { self.biomes.get(position) }

    /// Set the biome id at the given position within the section,
    /// returning the previous id.
    #[inline]
    #[must_use]
    pub fn set_raw_biome(&mut self, position: SectionBlockPos, id: u32) -> u32 {
        self.biomes.set(position, id)
    }

    /// Create an iterator over all raw biome ids in this section.
    #[inline]
    pub fn iter_raw_biomes(&self) -> impl Iterator<Item = u32> + '_ { self.biomes.iter() }

    /// Returns `true` if the given biome id is contained within this section.
    #[inline]
    #[must_use]
    pub fn contains_raw_biome(&self, id: u32) -> bool { self.biomes.contains(id) }
}

// ------------------------------------------------------------------------------------------------

/// A bit-packed bundle of chunk data.
#[derive(Default, Clone)]
pub struct SectionData<T: SectionType> {
    bits: usize,
    palette: SectionPalette,
    data: BitVec<u64, Msb0>,
    _phantom: PhantomData<T>,
}

impl<T: SectionType> SectionData<T> {
    /// Create a new [`SectionData`] without performing any validation.
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

    /// Set the value at the given position within the section,
    /// returning the previous value.
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "The index cannot ever go out of bounds")]
    pub fn set(&self, position: SectionBlockPos, id: u32) -> u32 {
        let width = usize::from(SECTION_WIDTH) / T::QUANTIZATION;
        let height = usize::from(SECTION_HEIGHT) / T::QUANTIZATION;

        self.set_index(
            (usize::from(position.x()) / T::QUANTIZATION)
                + (usize::from(position.z()) / T::QUANTIZATION * width)
                + (usize::from(position.y()) / T::QUANTIZATION * width * height),
            id,
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

    /// Set the value at the given index within the section,
    /// returning the previous value.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn set_index(&self, index: usize, _id: u32) -> Option<u32> {
        if index > usize::from(T::VOLUME) {
            return None;
        }

        todo!()
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

    /// Get a mutable reference to the entry at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the data is a valid entry in the section's
    /// palette.
    #[inline]
    #[must_use]
    pub unsafe fn get_slice_mut(&mut self, index: usize) -> Option<&mut BitSlice<u64, Msb0>> {
        let start = index * self.bits;
        self.data.get_mut(Range { start, end: start + self.bits })
    }

    /// Create an iterator over all values in this section.
    #[expect(clippy::missing_panics_doc, reason = "The index cannot ever go out of bounds")]
    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        (0..usize::from(T::VOLUME))
            .map(move |i| self.get_index(i).expect("Volume should always be within bounds"))
    }

    /// Returns `true` if the given id is contained within this section.
    #[must_use]
    pub fn contains(&self, id: u32) -> bool {
        match &self.palette {
            SectionPalette::Single(value) => *value == id,
            SectionPalette::Vector(items) => {
                if items.contains(&id) {
                    // Cannot return `true` directly as the palette may contain unused values.
                    self.iter().any(|value| value == id)
                } else {
                    false
                }
            }
            SectionPalette::Global => self.iter().any(|value| value == id),
        }
    }
}

// ------------------------------------------------------------------------------------------------

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
