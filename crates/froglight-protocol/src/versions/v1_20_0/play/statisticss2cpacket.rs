use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct StatisticsS2CPacket {
    // TODO: Figure out what this is
    pub stats: UnsizedBuffer,
}
