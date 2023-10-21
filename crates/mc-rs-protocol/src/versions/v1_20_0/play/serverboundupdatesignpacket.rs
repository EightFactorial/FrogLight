use compact_str::CompactString;
use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ServerboundUpdateSignPacket {
    pub position: BlockPos,
    pub front: bool,
    pub lines: [CompactString; 4],
}
