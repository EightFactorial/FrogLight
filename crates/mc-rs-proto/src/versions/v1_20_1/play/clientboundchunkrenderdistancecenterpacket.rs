use mc_rs_macros::Transcode;

use crate::types::position::ChunkPos;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkRenderDistanceCenterPacket {
    #[var]
    pub position: ChunkPos,
}
