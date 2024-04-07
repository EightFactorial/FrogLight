use froglight_macros::FrogReadWrite;

use crate::common::ServerStatus;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct QueryResponseS2CPacket {
    pub status: ServerStatus,
}
