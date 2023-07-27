use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBlockEventPacket {
    pub position: BlockPos,
    pub action_id: u8,
    pub action_parameter: u8,
    #[var]
    pub block_state: u32,
}
