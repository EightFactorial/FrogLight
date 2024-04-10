use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct TeamS2CPacket {
    pub name: CompactString,
    // TODO: Implement team data
    pub data: UnsizedBuffer,
}
