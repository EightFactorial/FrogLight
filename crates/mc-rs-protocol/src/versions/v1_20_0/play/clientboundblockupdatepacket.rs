use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBlockUpdatePacket {
    pub position: BlockPos,
    #[var]
    pub block_state: u32,
}
