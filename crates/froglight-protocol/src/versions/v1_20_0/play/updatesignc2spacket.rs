use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::BlockPosition;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct UpdateSignC2SPacket {
    pub position: BlockPosition,
    pub front: bool,
    pub lines: [CompactString; 4],
}
