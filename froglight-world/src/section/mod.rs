//! TODO

use core::marker::PhantomData;

use bit_vec::BitVec;
use smallvec::SmallVec;

mod traits;
pub use traits::*;

use crate::{SECTION_HEIGHT, SECTION_WIDTH, component::SectionBlockPos};

/// A piece of a chunk.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Section {
    block_count: u16,
    fluid_count: u16,
    blocks: SectionData<BlockSection>,
    biomes: SectionData<BiomeSection>,
}

impl Section {
    /// An empty [`Section`].
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self {
            block_count: 0,
            fluid_count: 0,
            blocks: SectionData::empty(),
            biomes: SectionData::empty(),
        }
    }

    /// Create a new [`Section`] without performing any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided data is valid.
    #[must_use]
    pub const unsafe fn new_unchecked(
        block_count: u16,
        fluid_count: u16,
        blocks: SectionData<BlockSection>,
        biomes: SectionData<BiomeSection>,
    ) -> Self {
        Self { block_count, fluid_count, blocks, biomes }
    }

    /// Get the number of non-air blocks in this section.
    #[must_use]
    pub const fn block_count(&self) -> u16 { self.block_count }

    /// Get the number of fluid blocks in this section.
    #[must_use]
    pub const fn fluid_count(&self) -> u16 { self.fluid_count }

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
        mut is_fluid: impl FnMut(u32) -> bool,
    ) -> u32 {
        let previous = self.blocks.set(position, block_id);
        match (is_air(previous), is_air(block_id)) {
            // Non-air to air, decrement block counter.
            (false, true) => self.block_count -= 1,
            // Air to non-air, increment block counter.
            (true, false) => self.block_count += 1,
            _ => {}
        }
        match (is_fluid(previous), is_fluid(block_id)) {
            // Non-fluid to fluid, increment fluid counter.
            (false, true) => self.fluid_count += 1,
            // Fluid to non-fluid, decrement fluid counter.
            (true, false) => self.fluid_count -= 1,
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
#[derive(Default, Clone, PartialEq, Eq)]
pub struct SectionData<T: SectionType> {
    bits: usize,
    palette: SectionPalette,
    data: BitVec<u64>,
    _phantom: PhantomData<T>,
}

impl<T: SectionType> SectionData<T> {
    /// An empty [`SectionData`].
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self {
            bits: 0,
            palette: SectionPalette::Single(0),
            data: BitVec::new_general(),
            _phantom: PhantomData,
        }
    }

    /// Create a new [`SectionData`] without performing any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided data is valid.
    #[must_use]
    pub const unsafe fn new_unchecked(
        bits: usize,
        palette: SectionPalette,
        data: BitVec<u64>,
    ) -> Self {
        Self { bits, palette, data, _phantom: PhantomData }
    }

    /// Get the number of bits per entry.
    #[must_use]
    pub const fn bits_per_entry(&self) -> usize { self.bits }

    /// Get the palette used by this data.
    #[must_use]
    pub const fn palette(&self) -> &SectionPalette { &self.palette }

    /// Get the palette used by this data mutably.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the underlying data is still valid after the
    /// palette is modified.
    #[must_use]
    pub const unsafe fn palette_mut(&mut self) -> &mut SectionPalette { &mut self.palette }

    /// Get the raw bit data.
    #[inline]
    #[must_use]
    pub const fn data(&self) -> &BitVec<u64> { &self.data }

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
    pub fn set(&mut self, position: SectionBlockPos, id: u32) -> u32 {
        let width = usize::from(SECTION_WIDTH) / T::QUANTIZATION;
        let height = usize::from(SECTION_HEIGHT) / T::QUANTIZATION;

        self.set_index(
            (usize::from(position.x()) / T::QUANTIZATION)
                + (usize::from(position.z()) / T::QUANTIZATION * width)
                + (usize::from(position.y()) / T::QUANTIZATION * width * height),
            id,
        )
        .expect("SectionBlockPos should always be within bounds?!")
    }

    /// Get the value at the given index within the section.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<u32> {
        if index > usize::from(T::VOLUME) {
            return None;
        }

        // Use the value from the palette directly.
        if let SectionPalette::Single(value) = self.palette {
            return Some(value);
        }

        // Read the value from the bit-packed data.
        let index = self.read_raw_index(index)?;
        match &self.palette {
            SectionPalette::Single(_) => unreachable!(),
            SectionPalette::Vector(items) => {
                let index = usize::try_from(index).ok()?;
                items.get(index).copied()
            }
            SectionPalette::Global => Some(index),
        }
    }

    /// Set the value at the given index within the section,
    /// returning the previous value.
    ///
    /// Returns `None` if the index is out of bounds.
    #[allow(clippy::must_use_candidate, reason = "Not required")]
    pub fn set_index(&mut self, index: usize, id: u32) -> Option<u32> {
        if index > usize::from(T::VOLUME) {
            return None;
        }

        match &mut self.palette {
            SectionPalette::Single(value) => {
                let value = *value;
                if id == value {
                    return Some(value);
                }

                // Convert `SectionPalette::Single` to `SectionPalette::Vector`.
                let mut palette = SmallVec::new_const();
                palette.push(value);
                palette.push(id);
                self.palette = SectionPalette::Vector(palette);

                self.grow_bitvec(1);
                self.write_raw_index(index, 1);

                Some(value)
            }

            // TODO: Convert `SectionPalette::Vector` to `SectionPalette::Global`.
            #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
            SectionPalette::Vector(items) => {
                let id = items.iter().position(|v| *v == id).unwrap_or_else(|| {
                    items.push(id);
                    items.len() - 1
                });

                let previous = self.read_raw_index(index)?;
                self.write_raw_index(index, id as u32).then_some(previous)
            }

            #[expect(clippy::cast_possible_truncation, reason = "Ignored")]
            SectionPalette::Global => {
                // Grow the bitvec to fit the new id if necessary.
                if id.bit_width() > self.bits as u32 {
                    self.grow_bitvec(id.bit_width() as usize);
                }

                let previous = self.read_raw_index(index)?;
                self.write_raw_index(index, id).then_some(previous)
            }
        }
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

impl<T: SectionType> SectionData<T> {
    /// Read a [`u32`] starting at the given value-index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[inline]
    fn read_raw_index(&self, index: usize) -> Option<u32> {
        Self::read_bitvec_index(&self.data, self.bits, index)
    }

    /// Read a [`u32`] starting at the given value-index.
    ///
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    fn read_bitvec_index(bitvec: &BitVec<u64>, bits: usize, index: usize) -> Option<u32> {
        let start = index * bits;
        let end = start + bits;

        if end > bitvec.len() {
            return None;
        }

        let mut value = 0u32;
        for n in 0..bits {
            if bitvec.get(start + n).unwrap_or(false) {
                value |= 1 << n;
            }
        }

        Some(value)
    }

    /// Write a [`u32`] starting at the given value-index.
    ///
    /// Returns `false` if the index was out of bounds.
    #[inline]
    #[allow(clippy::must_use_candidate, reason = "Not required")]
    fn write_raw_index(&mut self, index: usize, value: u32) -> bool {
        Self::write_bitvec_index(&mut self.data, self.bits, index, value)
    }

    /// Write a [`u32`] starting at the given value-index.
    ///
    /// Returns `false` if the index was out of bounds.
    #[allow(clippy::must_use_candidate, reason = "Not required")]
    fn write_bitvec_index(bitvec: &mut BitVec<u64>, bits: usize, index: usize, value: u32) -> bool {
        let start = index * bits;
        let end = start + bits;
        if end > bitvec.len() {
            return false;
        }

        for n in 0..bits {
            bitvec.set(start + n, (value & (1 << n)) != 0);
        }

        true
    }

    /// Grow the underlying [`BitVec`] to the given number of bits per entry.
    ///
    /// # Panics
    ///
    /// Panics if attempting to shrink the number of bits per entry.
    fn grow_bitvec(&mut self, bits: usize) {
        assert!(self.bits <= bits, "Cannot shrink the number of bits per entry!");

        // Do nothing if the number is the same.
        if self.bits == bits {
            return;
        }

        let mut output = BitVec::from_elem_general(usize::from(T::VOLUME) * bits, false);

        // Skip reading/writing if the current bitvec is empty.
        if self.bits == 0 {
            self.bits = bits;
            self.data = output;
            return;
        }

        // Read each value and write it to the new bitvec.
        for index in (0..T::VOLUME).map(usize::from) {
            if let Some(value) = self.read_raw_index(index) {
                Self::write_bitvec_index(&mut output, bits, index, value);
            }
        }

        self.bits = bits;
        self.data = output;
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
