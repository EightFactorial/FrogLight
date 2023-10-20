use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 128, 1])]
pub struct ClientboundBlockUpdatePacket {
    pub position: BlockPos,
    #[var]
    pub block_state: u32,
}
