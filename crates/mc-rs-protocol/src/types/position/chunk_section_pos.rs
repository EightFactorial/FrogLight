use bevy_math::IVec3;
use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Test;

use crate::buffer::{Decode, Encode};

use super::ChunkPos;

/// A chunk section position.
///
/// This is a chunk's position and a section's height.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, Test)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ChunkSectionPos(pub IVec3);

impl ChunkSectionPos {
    pub const ZERO: Self = Self(IVec3::ZERO);

    /// Creates a new [`ChunkSectionPos`].
    pub const fn new(x: i32, y: i32, z: i32) -> Self { Self(IVec3::new(x, y, z)) }
}

impl From<ChunkSectionPos> for ChunkPos {
    fn from(value: ChunkSectionPos) -> Self { ChunkPos::new(value.x, value.z) }
}

impl Encode for ChunkSectionPos {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        let long = ((self.x as i64 & 0x3FFFFF) << 42)
            | (self.y as i64 & 0xFFFFF)
            | ((self.z as i64 & 0x3FFFFF) << 20);

        long.encode(buf)
    }
}

impl Decode for ChunkSectionPos {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let long = i64::decode(buf)?;

        Ok(Self::new(
            (long >> 42) as i32,
            (long << 44 >> 44) as i32,
            (long << 22 >> 42) as i32,
        ))
    }
}
