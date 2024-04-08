use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct ChatMessageC2SPacket {
    pub message: CompactString,
    pub timestamp: u64,
    pub salt: u64,
    // TODO: Parse this
    pub data: UnsizedBuffer,
}
