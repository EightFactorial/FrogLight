use bevy_math::IVec3;
use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Test;

use crate::buffer::{Decode, Encode};

use super::ChunkPos;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, Test)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct BlockPos(pub IVec3);

impl BlockPos {
    const PACKED_X_LENGTH: u64 = 1 + 25; // minecraft does something a bit more complicated to get this 25
    const PACKED_Z_LENGTH: u64 = Self::PACKED_X_LENGTH;
    const PACKED_Y_LENGTH: u64 = 64 - Self::PACKED_X_LENGTH - Self::PACKED_Z_LENGTH;
    const PACKED_X_MASK: u64 = (1 << Self::PACKED_X_LENGTH) - 1;
    const PACKED_Y_MASK: u64 = (1 << Self::PACKED_Y_LENGTH) - 1;
    const PACKED_Z_MASK: u64 = (1 << Self::PACKED_Z_LENGTH) - 1;
    const Z_OFFSET: u64 = Self::PACKED_Y_LENGTH;
    const X_OFFSET: u64 = Self::PACKED_Y_LENGTH + Self::PACKED_Z_LENGTH;

    pub const ZERO: Self = Self(IVec3::ZERO);

    pub const fn new(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }

    pub fn from_chunk_pos(chunk_pos: ChunkPos, y: i32) -> Self {
        Self::new(chunk_pos.x * 16, y, chunk_pos.y * 16)
    }
}

impl Encode for BlockPos {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        let mut val: u64 = 0;
        val |= ((self.x as u64) & Self::PACKED_X_MASK) << Self::X_OFFSET;
        val |= (self.y as u64) & Self::PACKED_Y_MASK;
        val |= ((self.z as u64) & Self::PACKED_Z_MASK) << Self::Z_OFFSET;
        val.encode(buf)
    }
}

impl Decode for BlockPos {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let val = i64::decode(buf)?;

        let x = (val << (64 - Self::X_OFFSET - Self::PACKED_X_LENGTH)
            >> (64 - Self::PACKED_X_LENGTH)) as i32;

        let y = (val << (64 - Self::PACKED_Y_LENGTH) >> (64 - Self::PACKED_Y_LENGTH)) as i32;

        let z = (val << (64 - Self::Z_OFFSET - Self::PACKED_Z_LENGTH)
            >> (64 - Self::PACKED_Z_LENGTH)) as i32;

        Ok(BlockPos::new(x, y, z))
    }
}
