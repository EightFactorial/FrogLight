use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ChatCommandSignedPacket {
    pub command: CompactString,
    pub timestamp: u64,
    pub salt: i64,
    // TODO: Implement CommandData
    pub data: UnsizedBuffer,
}
