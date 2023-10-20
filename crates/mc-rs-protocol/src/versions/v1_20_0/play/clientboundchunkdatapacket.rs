use std::fmt::Debug;

use mc_rs_macros::Transcode;

use crate::types::{packets::chunk_data::ChunkDataPacket, position::ChunkPos, UnsizedByteBuffer};

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Clone, Transcode)]
pub struct ClientboundChunkDataPacket {
    pub position: ChunkPos,
    pub chunk_data: ChunkDataPacket,
    pub light_data: UnsizedByteBuffer,
}

impl Debug for ClientboundChunkDataPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientboundChunkDataPacket")
            .field("position", &self.position)
            .field("chunk_data", &self.chunk_data)
            .finish()
    }
}
