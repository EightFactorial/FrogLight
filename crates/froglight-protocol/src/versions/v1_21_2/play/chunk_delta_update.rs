//! @generated by `froglight-generator` #e606248

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChunkDeltaUpdatePacket {
    pub field_0: i64,
    #[frog(var)]
    pub field_1: u32,
    #[frog(var)]
    pub field_2: u64,
}
