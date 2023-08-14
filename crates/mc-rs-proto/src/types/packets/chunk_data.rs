use std::fmt::Debug;

use azalea_nbt::Nbt;
use mc_rs_macros::Transcode;

use crate::{
    buffer::{Decode, Encode, VarDecode, VarEncode},
    types::position::ChunkBlockPos,
};

#[derive(Clone, Transcode)]
pub struct ChunkDataPacket {
    pub heightmaps: Nbt,
    pub data: Vec<u8>,
    pub entities: Vec<BlockEntity>,
}

impl Debug for ChunkDataPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkDataPacket")
            .field("entities", &self.entities)
            .finish()
    }
}

#[derive(Clone, PartialEq)]
pub struct BlockEntity {
    pub position: ChunkBlockPos,
    pub kind: u32,
    pub data: Nbt,
}

impl Debug for BlockEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockEntity")
            .field("position", &self.position)
            .field("kind", &self.kind)
            .finish()
    }
}

impl Encode for BlockEntity {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        let mut byte = 0;
        byte |= (self.position.x & 0x0F) << 4;
        byte |= self.position.z & 0x0F;
        byte.encode(buf)?;

        i16::try_from(self.position.y)?.encode(buf)?;
        self.kind.var_encode(buf)?;
        self.data.encode(buf)?;
        Ok(())
    }
}

impl Decode for BlockEntity {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let byte = u8::decode(buf)?;

        Ok(Self {
            position: ChunkBlockPos {
                x: (byte >> 4) & 0x0F,
                y: i16::decode(buf)?.into(),
                z: byte & 0x0F,
            },
            kind: u32::var_decode(buf)?,
            data: Nbt::decode(buf)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SectionDataPacket {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub state: u32,
}

impl Encode for SectionDataPacket {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        let long: u64 = (self.state as u64) << 12
            | (u64::from(self.x) << 8)
            | (u64::from(self.z) << 4)
            | u64::from(self.y);

        long.var_encode(buf)
    }
}

impl Decode for SectionDataPacket {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let long = u64::var_decode(buf)?;

        log::debug!("long: {:b}", long);

        Ok(Self {
            x: ((long >> 8) & 0x0F) as u8,
            y: (long & 0x0F) as u8,
            z: ((long >> 4) & 0x0F) as u8,
            state: (long >> 12) as u32,
        })
    }
}
