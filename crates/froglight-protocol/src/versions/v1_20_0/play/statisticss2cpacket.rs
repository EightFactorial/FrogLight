use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct StatisticsS2CPacket {
    // TODO: Figure out what this is
    pub stats: UnsizedByteBuffer,
}
