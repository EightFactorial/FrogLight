use mc_rs_macros::Transcode;

use crate::types::{position::ChunkPos, UnsizedByteBuffer};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLightUpdatePacket {
    #[var]
    pub position: ChunkPos,
    pub data: UnsizedByteBuffer,
}
