use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateSignPacket {
    pub position: BlockPos,
    pub front: bool,
    pub lines: [String; 4],
}
