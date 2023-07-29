use fastnbt::Value;
use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

#[derive(Debug, Clone, Transcode)]
pub struct ChunkData {
    pub heightmaps: Value,
    pub data: Vec<u8>,
    pub entities: Vec<BlockEntity>,
}

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct BlockEntity {
    pub position: u8,
    pub y: u16,
    pub kind: ResourceLocation,
    pub data: Value,
}
