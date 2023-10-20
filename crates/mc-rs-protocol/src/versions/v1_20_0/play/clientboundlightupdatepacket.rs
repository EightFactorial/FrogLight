use mc_rs_macros::Transcode;

use crate::types::{position::ChunkPos, UnsizedByteBuffer};

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundLightUpdatePacket {
    #[var]
    pub position: ChunkPos,
    pub data: UnsizedByteBuffer,
}
