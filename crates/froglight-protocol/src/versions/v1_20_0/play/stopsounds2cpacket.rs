use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct StopSoundS2CPacket {
    // TODO: Implement sound types and categories
    pub data: UnsizedByteBuffer,
}
