//! TODO

#![allow(dead_code, unreachable_code, unreachable_pub)]

use bitvec::{order::Msb0, vec::BitVec};

/// The internal block and biome data of a chunk.
#[derive(Default, Clone)]
pub struct Section {
    /// The number of non-air blocks in the section.
    blocks: u32,
    /// Binary block data.
    blockdata: BitVec<u64, Msb0>,
    /// Binary biome data.
    biomedata: BitVec<u64, Msb0>,
}

#[allow(unused_assignments, unused_variables)]
impl Section {
    /// The total volume of the section in blocks.
    pub const VOLUME: usize = Self::HEIGHT * Self::WIDTH * Self::DEPTH;
    /// The height of the section in blocks.
    pub const HEIGHT: usize = 16;
    /// The width of the section in blocks.
    pub const WIDTH: usize = 16;
    /// The depth of the section in blocks.
    pub const DEPTH: usize = 16;

    /// Get a block from the section.
    #[must_use]
    pub fn get_block(&self, mut x: usize, mut y: usize, mut z: usize) -> u32 {
        x %= Section::WIDTH;
        y %= Section::HEIGHT;
        z %= Section::DEPTH;

        todo!()
    }

    /// Set a block in the section.
    pub fn set_block(&mut self, mut x: usize, mut y: usize, mut z: usize, block: u32) -> u32 {
        x %= Section::WIDTH;
        y %= Section::HEIGHT;
        z %= Section::DEPTH;

        let result = todo!();

        // Increase/decrease the non-air block count.
        if block == 0 && result != 0 {
            self.blocks -= 1;
        } else if block != 0 && result == 0 {
            self.blocks += 1;
        }

        result
    }
}
