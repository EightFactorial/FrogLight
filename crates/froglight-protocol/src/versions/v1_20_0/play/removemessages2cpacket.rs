use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct RemoveMessageS2CPacket {
    // TODO: Implement this
    pub signature: UnsizedBuffer,
}
