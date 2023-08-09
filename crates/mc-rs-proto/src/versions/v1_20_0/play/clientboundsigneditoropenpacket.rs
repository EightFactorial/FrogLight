use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSignEditorOpenPacket {
    pub position: BlockPos,
    pub front: bool,
}
