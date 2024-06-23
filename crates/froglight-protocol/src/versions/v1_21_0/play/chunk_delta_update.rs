use froglight_macros::FrogReadWrite;

use crate::{common::SectionPosition, packet::SectionDataPacket};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChunkDeltaUpdatePacket {
    pub position: SectionPosition,
    pub updates: Vec<SectionDataPacket>,
}
