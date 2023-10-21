use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ServerboundJigsawGeneratingPacket {
    pub position: BlockPos,
    #[var]
    pub levels: u32,
    pub keep_jigsaws: bool,
}
