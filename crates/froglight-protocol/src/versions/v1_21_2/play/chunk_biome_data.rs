//! @generated by `froglight-generator` #e606248

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChunkBiomeDataPacket {
    pub field_0: ChunkPos,
    pub field_1: Vec<u8>,
    pub field_2: Vec<()>,
}