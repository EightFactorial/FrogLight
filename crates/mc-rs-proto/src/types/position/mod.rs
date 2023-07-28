use bevy_math::{IVec2, IVec3, Vec3};

mod block_pos;
pub use block_pos::BlockPos;

mod chunk_pos;
pub use chunk_pos::ChunkPos;

mod chunk_block_pos;
pub use chunk_block_pos::ChunkBlockPos;

mod chunk_section_pos;
pub use chunk_section_pos::ChunkSectionPos;

mod global_pos;
pub use global_pos::GlobalPos;

use crate::buffer::{Decode, Encode};

impl Encode for Vec3 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        self.x.encode(buf)?;
        self.y.encode(buf)?;
        self.z.encode(buf)
    }
}

impl Decode for Vec3 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        Ok(Self {
            x: Decode::decode(buf)?,
            y: Decode::decode(buf)?,
            z: Decode::decode(buf)?,
        })
    }
}

impl Encode for IVec2 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        self.x.encode(buf)?;
        self.y.encode(buf)
    }
}

impl Decode for IVec2 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        Ok(Self {
            x: Decode::decode(buf)?,
            y: Decode::decode(buf)?,
        })
    }
}

impl Encode for IVec3 {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        self.x.encode(buf)?;
        self.y.encode(buf)?;
        self.z.encode(buf)
    }
}

impl Decode for IVec3 {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        Ok(Self {
            x: Decode::decode(buf)?,
            y: Decode::decode(buf)?,
            z: Decode::decode(buf)?,
        })
    }
}
