use mc_rs_macros::Transcode;

use crate::types::{packets::chunk_data::ChunkDataPacket, position::ChunkPos, UnsizedByteBuffer};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkDataPacket {
    pub position: ChunkPos,
    pub chunk_data: ChunkDataPacket,
    pub light_data: UnsizedByteBuffer,
}
