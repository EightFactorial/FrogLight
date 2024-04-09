use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct PlayerSessionC2SPacket {
    // TODO: Parse this
    pub data: UnsizedBuffer,
}
