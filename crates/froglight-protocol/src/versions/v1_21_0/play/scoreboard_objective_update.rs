use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [4, 84, 101, 115, 116, 1])]
pub struct ScoreboardObjectiveUpdatePacket {
    pub objective: CompactString,
    // TODO: Implement ObjectiveData
    pub data: UnsizedBuffer,
}
