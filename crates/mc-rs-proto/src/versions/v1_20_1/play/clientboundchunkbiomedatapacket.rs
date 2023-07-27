use mc_rs_macros::Transcode;

use crate::types::packets::biome_data::ChunkBiomeData;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkBiomeDataPacket {
    pub biome_data: Vec<ChunkBiomeData>,
}
