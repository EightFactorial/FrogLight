use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1, 10, 109, 99, 45, 114, 115, 58, 116, 101, 115, 116, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct SynchronizeTagsS2CPacket {
    // TODO: Implement tags
    pub groups: UnsizedByteBuffer,
}
