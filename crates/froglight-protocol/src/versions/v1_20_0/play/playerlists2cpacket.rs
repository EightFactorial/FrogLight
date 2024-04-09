use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct PlayerListS2CPacket {
    pub actions: u8,
    pub data: UnsizedBuffer,
}
