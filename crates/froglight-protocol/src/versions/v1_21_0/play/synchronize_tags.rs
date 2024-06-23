use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct SynchronizeTagsPacket {
    // TODO: Implemenet TagData
    pub data: UnsizedBuffer,
}
