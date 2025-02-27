//! TODO

#![allow(dead_code, unreachable_code, unreachable_pub)]

use bitvec::{order::Msb0, vec::BitVec};
use glam::IVec3;

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
    /// The depth of the section in blocks.
    pub const DEPTH: usize = 16;
    /// The height of the section in blocks.
    pub const HEIGHT: usize = 16;
    /// The total volume of the section in blocks.
    pub const VOLUME: usize = Self::HEIGHT * Self::WIDTH * Self::DEPTH;
    /// The width of the section in blocks.
    pub const WIDTH: usize = 16;

    /// Get a block from the section.
    #[must_use]
    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn get_block(&self, mut position: IVec3) -> u32 {
        position.x %= Section::WIDTH as i32;
        position.y %= Section::HEIGHT as i32;
        position.z %= Section::DEPTH as i32;

        todo!()
    }

    /// Set a block in the section.
    #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn set_block(&mut self, mut position: IVec3, block: u32) -> u32 {
        position.x %= Section::WIDTH as i32;
        position.y %= Section::HEIGHT as i32;
        position.z %= Section::DEPTH as i32;

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
