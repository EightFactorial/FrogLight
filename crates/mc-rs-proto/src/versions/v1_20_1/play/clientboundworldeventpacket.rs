use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldEventPacket {
    pub event_type: u32,
    pub position: BlockPos,
    pub data: u32,
    pub global: bool,
}
