//! TODO
#![allow(dead_code, reason = "WIP")]

use core::marker::PhantomData;

use bitvec::{order::Msb0, slice::BitSlice};

use crate::component::SectionBlockPos;

/// A borrowed piece of a chunk.
#[derive(Default, Clone)]
pub struct BorrowedSection<'a> {
    solids: u32,
    blocks: BorrowedSectionData<'a, ()>,
    biomes: BorrowedSectionData<'a, ()>,
}

impl BorrowedSection<'_> {
    /// Get the block id at the given position within the section.
    #[must_use]
    pub fn get_raw_block(&self, _position: SectionBlockPos) -> u32 { todo!() }
}

// -------------------------------------------------------------------------------------------------

/// A bit-packed bundle of chunk data.
#[derive(Default, Clone)]
pub struct BorrowedSectionData<'a, T> {
    bits: usize,
    // palette: SectionPalette,
    data: &'a BitSlice<u64, Msb0>,
    _phantom: PhantomData<T>,
}
