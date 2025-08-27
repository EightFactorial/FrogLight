//! TODO

use core::{fmt::Debug, hash::Hash, marker::PhantomData, ops::Range};

use bitvec::{field::BitField, order::Lsb0, slice::BitSlice, vec::BitVec};

use crate::{palette::SectionPalette, position::RelativePosition};

/// A cube of block and biome data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Section {
    solids: u16,
    blocks: SectionData<Block>,
    biomes: SectionData<Biome>,
}

impl Section {
    /// The side length of a [`Section`] in blocks.
    pub const SIDE_LENGTH: usize = 16;
    /// The total number of blocks in a [`Section`].
    pub const VOLUME: usize = Self::SIDE_LENGTH * Self::SIDE_LENGTH * Self::SIDE_LENGTH;

    /// Returns the number of solid, non-air blocks in this [`Section`].
    #[inline]
    #[must_use]
    pub const fn solids(&self) -> u16 { self.solids }

    /// Returns `true` if this [`Section`] contains only air.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.solids == 0 }

    /// Get a block from this [`Section`].
    #[must_use]
    pub fn get_block(&self, position: RelativePosition) -> u32 {
        self.blocks.get(position.as_section_index())
    }

    /// Set a block in this [`Section`],
    /// updating the solid block count as necessary.
    ///
    /// Returns the previous block's ID.
    #[must_use]
    pub fn set_block(
        &mut self,
        block_id: u32,
        position: RelativePosition,
        is_air: impl Fn(u32) -> bool,
    ) -> u32 {
        let previous = self.blocks.set(position.as_section_index(), block_id);

        // Update the solid block count if necessary.
        match (is_air(previous), is_air(block_id)) {
            (true, false) => self.solids += 1,
            (false, true) => self.solids -= 1,
            _ => {}
        }

        previous
    }

    /// Get a biome from this [`Section`].
    #[must_use]
    pub fn get_biome(&self, position: RelativePosition) -> u32 {
        self.biomes.get(position.as_section_index())
    }

    /// Set a biome in this [`Section`].
    ///
    /// Returns the previous biome's ID.
    #[must_use]
    pub fn set_biome(&mut self, biome_id: u32, position: RelativePosition) -> u32 {
        self.biomes.set(position.as_section_index(), biome_id)
    }

    /// Get references to the inner [`Block`] and [`Biome`] data of this
    /// [`Section`].
    #[must_use]
    pub const fn data(section: &Self) -> (&SectionData<Block>, &SectionData<Biome>) {
        (&section.blocks, &section.biomes)
    }

    /// Get mutable references to the inner [`Block`] and [`Biome`] data of this
    /// [`Section`].
    #[must_use]
    pub fn data_mut(section: &mut Self) -> (&mut SectionData<Block>, &mut SectionData<Biome>) {
        (&mut section.blocks, &mut section.biomes)
    }

    /// Create a new [`Section`] from parts without validating them.
    ///
    /// # Safety
    ///
    /// TODO
    #[must_use]
    pub unsafe fn from_parts_unchecked(
        solids: u16,
        blocks: SectionData<Block>,
        biomes: SectionData<Biome>,
    ) -> Self {
        Self { solids, blocks, biomes }
    }
}

// -------------------------------------------------------------------------------------------------

/// A bit-packed array of [`Section`] data.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SectionData<T: SectionType> {
    bits: usize,
    data: BitVec<u64, Lsb0>,
    palette: SectionPalette,
    _phantom: PhantomData<T>,
}

impl<T: SectionType> SectionData<T> {
    /// Get an entry from this [`SectionData`].
    #[must_use]
    pub fn get(&self, mut index: usize) -> u32 {
        index %= T::ENTRIES;

        match &self.palette {
            SectionPalette::Single(block_id) => *block_id,
            SectionPalette::Vector(items) => {
                let slice = self.slice_at(index);
                match items.get(slice.load_le::<usize>()) {
                    Some(block_id) => *block_id,
                    None => 0, // TODO: Log an error or panic?,
                }
            }
            SectionPalette::Global => {
                let slice = self.slice_at(index);
                slice.load_le::<u32>()
            }
        }
    }

    /// Set an entry in this [`SectionData`] and return the previous entry.
    #[expect(clippy::must_use_candidate, reason = "You're not required to use the returned value")]
    pub fn set(&self, mut _index: usize, _data: u32) -> u32 {
        _index %= T::ENTRIES;

        todo!()
    }
}

impl<T: SectionType> SectionData<T> {
    /// Get a slice of bits corresponding to the given index.
    #[inline]
    #[must_use]
    pub fn slice_at(&self, index: usize) -> &BitSlice<u64, Lsb0> {
        Self::slice_of(index, self.bits, &self.data)
    }

    /// Get a mutable slice of bits corresponding to the given index.
    #[inline]
    #[must_use]
    pub fn slice_at_mut(&mut self, index: usize) -> &mut BitSlice<u64, Lsb0> {
        Self::slice_of_mut(index, self.bits, &mut self.data)
    }

    /// Get a slice of bits corresponding to the given index.
    #[must_use]
    fn slice_of(index: usize, bits: usize, data: &BitVec<u64, Lsb0>) -> &BitSlice<u64, Lsb0> {
        debug_assert!(index < T::ENTRIES, "Attempted to index beyond the section!");
        debug_assert_eq!(bits * T::ENTRIES, data.len(), "Data is not the correct size!");

        data.get(Range { start: index * bits, end: (index + 1) * bits })
            .unwrap_or_else(|| unreachable!("Index is guaranteed within bounds"))
    }

    /// Get a mutable slice of bits corresponding to the given index.
    #[must_use]
    fn slice_of_mut(
        index: usize,
        bits: usize,
        data: &mut BitVec<u64, Lsb0>,
    ) -> &mut BitSlice<u64, Lsb0> {
        debug_assert!(index < Section::VOLUME, "Attempted to index beyond the section!");
        debug_assert_eq!(bits * Section::VOLUME, data.len(), "Data is not the correct size!");

        data.get_mut(Range { start: index * bits, end: (index + 1) * bits })
            .unwrap_or_else(|| unreachable!("Index is guaranteed within bounds"))
    }

    /// Create a new [`SectionData`] from parts without validating them.
    ///
    /// # Safety
    ///
    /// TODO
    ///
    /// # Panics
    ///
    /// Panics if the length of `data` does not equal `bits * T::ENTRIES`.
    #[must_use]
    pub unsafe fn from_parts_unchecked(
        bits: usize,
        data: BitVec<u64, Lsb0>,
        palette: SectionPalette,
    ) -> Self {
        assert!(
            data.len() == bits * T::ENTRIES,
            "Data length ({}) does not match the expected size ({})!",
            data.len(),
            bits * T::ENTRIES
        );

        Self { bits, data, palette, _phantom: PhantomData }
    }
}

impl<T: SectionType> Debug for SectionData<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SectionData")
            .field("bits", &self.bits)
            .field("palette", &self.palette)
            .finish_non_exhaustive()
    }
}

// -------------------------------------------------------------------------------------------------

/// A sealed trait for [`Section`] data types.
pub trait SectionType: Debug + Default + Copy + Eq + Hash + sealed::Sealed {
    /// The number of entries in this section type.
    const ENTRIES: usize;

    /// Get the appropriate palette type for the given number of bits.
    fn palette_for_bits(bits: usize);
}

/// A [`SectionType`] for storing block data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block;

impl SectionType for Block {
    const ENTRIES: usize = Section::VOLUME;

    #[expect(clippy::match_same_arms, reason = "WIP")]
    fn palette_for_bits(bits: usize) {
        match bits {
            0 => (),    // SectionPalette::Single(0u32),
            1..9 => (), // SectionPalette::Vector(Vec::new()),
            _ => (),    // SectionPalette::Global,
        }
    }
}

/// A [`SectionType`] for storing biome data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Biome;

impl SectionType for Biome {
    const ENTRIES: usize = Section::VOLUME / 4;

    #[expect(clippy::match_same_arms, reason = "WIP")]
    fn palette_for_bits(bits: usize) {
        match bits {
            0 => (),    // SectionPalette::Single(0u32),
            1..4 => (), // SectionPalette::Vector(Vec::new()),
            _ => (),    // SectionPalette::Global,
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Block {}
    impl Sealed for super::Biome {}
}

// -------------------------------------------------------------------------------------------------

impl Section {
    /// A [`Section`] filled entirely with air.
    pub const AIR: Self = Self { solids: 0, blocks: SectionData::AIR, biomes: SectionData::AIR };

    /// A [`Section`] filled entirely with air.
    #[must_use]
    pub const fn air() -> Self { Self::AIR }
}

impl Default for Section {
    fn default() -> Self { Self::AIR }
}

impl<T: SectionType> SectionData<T> {
    /// A [`SectionData`] filled entirely with air.
    pub const AIR: Self =
        Self { bits: 0, data: BitVec::EMPTY, palette: SectionPalette::AIR, _phantom: PhantomData };

    /// A [`SectionData`] filled entirely with air.
    #[must_use]
    pub const fn air() -> Self { Self::AIR }
}

impl<T: SectionType> Default for SectionData<T> {
    fn default() -> Self { Self::AIR }
}
