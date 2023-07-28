use mc_rs_macros::Transcode;

use crate::types::{position::ChunkSectionPos, UnsizedByteBuffer};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkDeltaUpdatePacket {
    pub position: ChunkSectionPos,
    pub data: UnsizedByteBuffer,
}
