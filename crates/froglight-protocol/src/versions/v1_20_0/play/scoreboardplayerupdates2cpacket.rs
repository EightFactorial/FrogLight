use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [4, 84, 101, 115, 116, 1, 1, 4, 84, 101, 115, 116, 64])]
pub struct ScoreboardPlayerUpdateS2CPacket {
    pub player_name: CompactString,
    // TODO: Implement player update data
    pub data: UnsizedBuffer,
}
