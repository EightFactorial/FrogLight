use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct TeamS2CPacket {
    pub name: CompactString,
    // TODO: Implement team data
    pub data: UnsizedByteBuffer,
}
