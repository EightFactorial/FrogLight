use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Default, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct SynchronizeTagsS2CPacket {
    // TODO: Implement this
    pub groups: UnsizedByteBuffer,
}
