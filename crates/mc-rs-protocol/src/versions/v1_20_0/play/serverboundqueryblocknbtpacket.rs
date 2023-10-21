use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ServerboundQueryBlockNbtPacket {
    #[var]
    pub query_id: u32,
    pub position: BlockPos,
}
