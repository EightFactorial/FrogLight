use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CommandExecutionC2SPacket {
    pub command: CompactString,
    pub timestamp: u64,
    pub salt: u64,
    // TODO: Parse this
    pub data: UnsizedByteBuffer,
}
