use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 1])]
pub struct ClientboundBlockEventPacket {
    pub position: BlockPos,
    pub action_id: u8,
    pub action_parameter: u8,
    #[var]
    pub block_state: u32,
}
