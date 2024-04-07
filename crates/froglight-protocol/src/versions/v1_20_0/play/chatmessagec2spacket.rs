use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ChatMessageC2SPacket {
    pub message: CompactString,
    pub timestamp: u64,
    pub salt: u64,
    // TODO: Parse this
    pub data: UnsizedByteBuffer,
}
