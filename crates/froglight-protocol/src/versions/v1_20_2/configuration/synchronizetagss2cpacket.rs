use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Default, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct SynchronizeTagsS2CPacket {
    // TODO: Implement this
    pub groups: UnsizedBuffer,
}
