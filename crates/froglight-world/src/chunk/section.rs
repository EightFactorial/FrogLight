#![expect(dead_code)]

use std::marker::PhantomData;

use bitvec::{order::Msb0, vec::BitVec};

/// A bit-packed cube of block and biome data.
///
/// Contains both block and biome data.
#[derive(Default, Clone)]
pub struct Section {
    /// The number of non-air blocks in the section.
    blocks: u32,
    /// Binary block data.
    block: SectionData<Block>,
    /// Binary biome data.
    biome: SectionData<Biome>,
}

impl Section {
    /// The depth of a [`Section`] in blocks.
    pub const DEPTH: usize = Self::HEIGHT;
    /// The height of a [`Section`] in blocks.
    pub const HEIGHT: usize = 16;
    /// The width of a [`Section`] in blocks.
    pub const WIDTH: usize = Self::HEIGHT;

    /// Get the number of non-air blocks in the [`Section`].
    #[inline]
    #[must_use]
    pub const fn blocks(&self) -> u32 { self.blocks }

    /// Get the number of non-air blocks in the [`Section`] mutably.
    #[inline]
    #[must_use]
    pub const fn blocks_mut(&mut self) -> &mut u32 { &mut self.blocks }

    /// Get the block id at the given block index.
    #[must_use]
    pub fn get_block(&self, _index: usize) -> u32 { todo!() }

    /// Set the block id at the given block index.
    ///
    /// Returns the previous block id.
    ///
    /// # Warning
    /// This does not update the block count! This ***must*** be done manually!
    pub fn set_block(&mut self, _index: usize, _block_id: u32) -> u32 { todo!() }

    /// Get the biome id at the given block index.
    #[must_use]
    pub fn get_biome(&self, _index: usize) -> u32 { todo!() }

    /// Set the biome id at the given block index.
    ///
    /// Returns the previous biome id.
    pub fn set_biome(&mut self, _index: usize, _biome_id: u32) -> u32 { todo!() }
}

// -------------------------------------------------------------------------------------------------

/// A bit-packed cube of world data.
///
/// Contains either [`Block`] or [`Biome`] data.
#[derive(Default, Clone)]
#[expect(private_bounds)]
pub struct SectionData<T: SectionType> {
    bits: usize,
    // palette: SectionPalette,
    data: BitVec<u64, Msb0>,
    _phantom: PhantomData<T>,
}

#[expect(private_bounds)]
impl<T: SectionType> SectionData<T> {
    /// Get the number of bits used to store each entry.
    #[inline]
    #[must_use]
    pub const fn bits(&self) -> usize { self.bits }

    /// Get the number of bits used to store each entry mutably.
    #[inline]
    #[must_use]
    pub const fn bits_mut(&mut self) -> &mut usize { &mut self.bits }

    /// Get the raw section data.
    #[inline]
    #[must_use]
    pub const fn raw(&self) -> &BitVec<u64, Msb0> { &self.data }

    /// Get the raw section data mutably.
    #[inline]
    #[must_use]
    pub const fn raw_mut(&mut self) -> &mut BitVec<u64, Msb0> { &mut self.data }
}

use sealed::SectionType;
mod sealed {
    /// A type of [`Section`] storage.
    pub(crate) trait SectionType: Default + Clone + Send + Sync + 'static {
        // /// Get a [`SectionPalette`] for this number of bits.
        // fn palette_for(bits: usize) -> SectionPalette;
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Default, Clone, Copy)]
struct Biome;

impl SectionType for Biome {}

#[derive(Default, Clone, Copy)]
struct Block;

impl SectionType for Block {}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl froglight_io::standard::FrogRead for Section {
    fn frog_read(
        _buffer: &mut impl std::io::Read,
    ) -> Result<Self, froglight_io::prelude::ReadError> {
        todo!()
    }
}

#[cfg(feature = "io")]
impl froglight_io::standard::FrogWrite for Section {
    fn frog_write(
        &self,
        _buffer: &mut impl std::io::Write,
    ) -> Result<usize, froglight_io::prelude::WriteError> {
        todo!()
    }

    fn frog_len(&self) -> usize { todo!() }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl<T: SectionType> froglight_io::standard::FrogRead for SectionData<T> {
    fn frog_read(
        _buffer: &mut impl std::io::Read,
    ) -> Result<Self, froglight_io::prelude::ReadError> {
        todo!()
    }
}

#[cfg(feature = "io")]
impl<T: SectionType> froglight_io::standard::FrogWrite for SectionData<T> {
    fn frog_write(
        &self,
        _buffer: &mut impl std::io::Write,
    ) -> Result<usize, froglight_io::prelude::WriteError> {
        todo!()
    }

    fn frog_len(&self) -> usize { todo!() }
}
