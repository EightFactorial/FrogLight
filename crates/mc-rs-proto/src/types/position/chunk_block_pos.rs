use mc_rs_macros::Transcode;

use super::{BlockPos, ChunkPos};

/// A block's position in a chunk.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0])]
pub struct ChunkBlockPos {
    pub x: u8,
    pub y: i32,
    pub z: u8,
}

impl ChunkBlockPos {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    pub fn new(x: u8, y: i32, z: u8) -> Self { Self { x, y, z } }

    pub fn to_block_pos(self, chunk_pos: ChunkPos) -> BlockPos {
        BlockPos::new(
            chunk_pos.x * 16 + self.x as i32,
            self.y,
            chunk_pos.y * 16 + self.z as i32,
        )
    }
}

impl From<BlockPos> for ChunkBlockPos {
    fn from(value: BlockPos) -> Self {
        Self {
            x: (value.x % 16) as u8,
            y: value.y,
            z: (value.z % 16) as u8,
        }
    }
}
