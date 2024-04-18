use froglight_macros::FrogReadWrite;

use crate::{common::SectionPosition, packet::SectionDataPacket};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct ChunkDeltaUpdateS2CPacket {
    pub position: SectionPosition,
    pub updates: Vec<SectionDataPacket>,
}
