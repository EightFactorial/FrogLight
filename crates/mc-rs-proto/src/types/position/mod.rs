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

use crate::buffer::{Decode, Encode, VarDecode, VarEncode};

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

impl VarEncode for IVec2 {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        self.x.var_encode(buf)?;
        self.y.var_encode(buf)
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

impl VarDecode for IVec2 {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        Ok(Self {
            x: VarDecode::var_decode(buf)?,
            y: VarDecode::var_decode(buf)?,
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

impl VarEncode for IVec3 {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        self.x.var_encode(buf)?;
        self.y.var_encode(buf)?;
        self.z.var_encode(buf)
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

impl VarDecode for IVec3 {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        Ok(Self {
            x: VarDecode::var_decode(buf)?,
            y: VarDecode::var_decode(buf)?,
            z: VarDecode::var_decode(buf)?,
        })
    }
}
