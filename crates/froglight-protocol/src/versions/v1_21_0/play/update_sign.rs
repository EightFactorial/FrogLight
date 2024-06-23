use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct UpdateSignPacket {
    pub position: BlockPosition,
    pub front: bool,
    pub lines: [CompactString; 4],
}
