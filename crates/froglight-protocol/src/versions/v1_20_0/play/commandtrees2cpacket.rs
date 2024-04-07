use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CommandTreeS2CPacket {
    // TODO: Implement CommandTree
    pub data: UnsizedByteBuffer,
    // pub nodes: (),
    // pub root_size: (),
}
