use mc_rs_macros::Transcode;

use crate::types::position::ChunkPos;

#[derive(Debug, Clone, Transcode)]
pub struct ChunkBiomeData {
    pub position: ChunkPos,
    pub data: Vec<u8>,
}
