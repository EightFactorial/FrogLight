use fastnbt::Value;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ChunkDataPacket {
    pub heightmaps: Value,
    pub data: Vec<u8>,
    pub entities: Vec<BlockEntity>,
}

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct BlockEntity {
    pub position: u8,
    pub y: u16,
    #[var]
    pub kind: u32,
    pub data: Value,
}
