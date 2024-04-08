use froglight_macros::FrogReadWrite;

use crate::packet::ServerStatus;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct QueryResponseS2CPacket {
    pub status: ServerStatus,
}
