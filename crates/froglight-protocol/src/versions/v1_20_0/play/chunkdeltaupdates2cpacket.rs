use froglight_macros::FrogReadWrite;

use crate::common::{SectionDataPacket, SectionPosition};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ChunkDeltaUpdateS2CPacket {
    pub position: SectionPosition,
    pub updates: Vec<SectionDataPacket>,
}
