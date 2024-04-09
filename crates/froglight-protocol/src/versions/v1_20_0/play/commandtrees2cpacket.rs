use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct CommandTreeS2CPacket {
    // TODO: Implement CommandTree
    pub data: UnsizedBuffer,
    // pub nodes: (),
    // pub root_size: (),
}
