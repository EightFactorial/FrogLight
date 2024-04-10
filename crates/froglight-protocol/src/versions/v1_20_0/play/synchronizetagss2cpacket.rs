use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1, 10, 109, 99, 45, 114, 115, 58, 116, 101, 115, 116, 0])]
pub struct SynchronizeTagsS2CPacket {
    // TODO: Implement tags
    pub groups: UnsizedBuffer,
}
