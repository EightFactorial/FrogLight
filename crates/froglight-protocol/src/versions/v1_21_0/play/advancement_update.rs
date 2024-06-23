use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct AdvancementUpdatePacket {
    pub reset: bool,
    // TODO: Implement AdvancementData
    pub data: UnsizedBuffer,
}
