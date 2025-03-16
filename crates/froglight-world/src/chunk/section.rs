use bitvec::{order::Msb0, vec::BitVec};

#[derive(Default, Clone)]
pub struct Section {
    /// The number of non-air blocks in the section.
    blocks: u32,
    /// Binary block data.
    blockdata: BitVec<u64, Msb0>,
    /// Binary biome data.
    biomedata: BitVec<u64, Msb0>,
}

impl Section {
    /// The depth of a [`Section`] in blocks.
    pub const DEPTH: usize = 16;
    /// The height of a [`Section`] in blocks.
    pub const HEIGHT: usize = 16;
    /// The width of a [`Section`] in blocks.
    pub const WIDTH: usize = 16;
}
