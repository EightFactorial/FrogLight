use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct StopSoundS2CPacket {
    // TODO: Implement sound types and categories
    pub data: UnsizedBuffer,
}
