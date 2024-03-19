use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [4, 84, 101, 115, 116, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ScoreboardObjectiveUpdateS2CPacket {
    pub name: CompactString,
    // TODO: Implement objective type
    pub data: UnsizedByteBuffer,
}
