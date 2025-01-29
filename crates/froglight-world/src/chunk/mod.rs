//! TODO
#![allow(dead_code, unreachable_code, unreachable_pub)]

use bitvec::{order::Msb0, vec::BitVec};

/// A chunk of blocks in a world.
#[derive()]
struct Chunk<const SECTIONS: usize, const OFFSET: isize>([Section; SECTIONS]);

impl<const SECTIONS: usize, const OFFSET: isize> Chunk<SECTIONS, OFFSET> {
    /// The total volume of the chunk in blocks.
    pub const VOLUME: usize = Self::HEIGHT * Self::WIDTH * Self::DEPTH;
    /// The height of the chunk in blocks.
    pub const HEIGHT: usize = Section::HEIGHT * SECTIONS;
    /// The width of the chunk in blocks.
    pub const WIDTH: usize = Section::WIDTH;
    /// The depth of the chunk in blocks.
    pub const DEPTH: usize = Section::DEPTH;

    /// The total volume of the chunk in blocks.
    #[expect(clippy::unused_self)]
    pub const fn volume(&self) -> usize { Self::VOLUME }
    /// The height of the chunk in blocks.
    #[expect(clippy::unused_self)]
    pub const fn height(&self) -> usize { Self::HEIGHT }
    /// The width of the chunk in blocks.
    #[expect(clippy::unused_self)]
    pub const fn width(&self) -> usize { Self::WIDTH }
    /// The depth of the chunk in blocks.
    #[expect(clippy::unused_self)]
    pub const fn depth(&self) -> usize { Self::DEPTH }

    /// Get a reference to a section based on the y coordinate.
    ///
    /// # Note
    /// This does not take into account the chunk offset.
    #[inline]
    #[must_use]
    fn get_section(&self, y: usize) -> Option<&Section> { self.0.get(y / Self::HEIGHT) }
    /// Get a mutable reference to a section based on the y coordinate.
    ///
    /// # Note
    /// This does not take into account the chunk offset.
    #[inline]
    #[must_use]
    fn get_section_mut(&mut self, y: usize) -> Option<&mut Section> {
        self.0.get_mut(y / Self::HEIGHT)
    }

    /// Get a block from the chunk.
    ///
    /// Returns `None` if the y coordinate is out of bounds.
    #[must_use]
    pub fn get_block_raw(&self, x: usize, mut y: usize, z: usize) -> Option<u32> {
        y = y.checked_add_signed(-OFFSET)?;
        self.get_section(y).map(|s| s.get_block(x, y, z))
    }
    /// Set a block in the chunk.
    ///
    /// Returns `None` if the y coordinate is out of bounds.
    #[must_use]
    pub fn set_block_raw(&mut self, x: usize, mut y: usize, z: usize, block: u32) -> Option<u32> {
        y = y.checked_add_signed(-OFFSET)?;
        self.get_section_mut(y).map(|s| s.set_block(x, y, z, block))
    }
}

#[derive(Default)]
struct Section {
    /// The number of non-air blocks in the section.
    blocks: u32,
    blockdata: BitVec<u64, Msb0>,
    biomedata: BitVec<u64, Msb0>,
}

#[allow(unused_assignments, unused_variables)]
impl Section {
    /// The height of the section in blocks.
    pub const HEIGHT: usize = 16;
    /// The width of the section in blocks.
    pub const WIDTH: usize = 16;
    /// The depth of the section in blocks.
    pub const DEPTH: usize = 16;

    pub fn get_block(&self, mut x: usize, mut y: usize, mut z: usize) -> u32 {
        x %= Section::WIDTH;
        y %= Section::HEIGHT;
        z %= Section::DEPTH;

        todo!()
    }

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

#[test]
fn test() {
    let overworld_chunk = Chunk::<24, -64>(std::array::from_fn(|_| Section::default()));
    println!("24: Height: {}, Volume: {}", overworld_chunk.height(), overworld_chunk.volume());
    let nether_chunk = Chunk::<16, 0>(std::array::from_fn(|_| Section::default()));
    println!("16: Height: {}, Volume: {}", nether_chunk.height(), nether_chunk.volume());
}
