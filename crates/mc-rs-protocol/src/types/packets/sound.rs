use std::io::Write;

use mc_rs_macros::Test;

use crate::{
    buffer::{Decode, Encode, EncodeError, VarDecode, VarEncode},
    types::ResourceLocation,
};

#[derive(Debug, Clone, PartialEq, Eq, Test)]
#[mctest(tests = ["transcode", "decode"], bytes = [1])]
pub enum PacketSoundType {
    SoundId(u32),
    SoundName {
        registry: ResourceLocation,
        fixed_range: bool,
    },
}

impl Encode for PacketSoundType {
    fn encode(&self, buf: &mut impl Write) -> Result<(), EncodeError> {
        match self {
            PacketSoundType::SoundId(id) => (id + 1).var_encode(buf),
            PacketSoundType::SoundName {
                registry,
                fixed_range,
            } => {
                registry.encode(buf)?;
                fixed_range.encode(buf)
            }
        }
    }
}

impl Decode for PacketSoundType {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        match u32::var_decode(buf)? {
            0 => Ok(PacketSoundType::SoundName {
                registry: ResourceLocation::decode(buf)?,
                fixed_range: bool::decode(buf)?,
            }),
            id => Ok(PacketSoundType::SoundId(id - 1)),
        }
    }
}
