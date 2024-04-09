use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct CommandExecutionC2SPacket {
    pub command: CompactString,
    pub timestamp: u64,
    pub salt: u64,
    // TODO: Parse this
    pub data: UnsizedBuffer,
}
