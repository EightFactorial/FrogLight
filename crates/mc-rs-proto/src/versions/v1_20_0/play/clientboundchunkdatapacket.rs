use mc_rs_macros::Transcode;

use crate::types::{position::ChunkPos, UnsizedByteBuffer};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkDataPacket {
    pub position: ChunkPos,
    pub data: UnsizedByteBuffer,
    // pub chunk_data: UnsizedByteBuffer,
    // pub light_data: UnsizedByteBuffer,
}
