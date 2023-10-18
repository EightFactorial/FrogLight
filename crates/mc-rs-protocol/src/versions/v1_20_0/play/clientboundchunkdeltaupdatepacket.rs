use mc_rs_macros::Transcode;

use crate::types::{packets::chunk_data::SectionDataPacket, position::ChunkSectionPos};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkDeltaUpdatePacket {
    pub position: ChunkSectionPos,
    pub updates: Vec<SectionDataPacket>,
}
