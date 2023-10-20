use mc_rs_macros::Transcode;

use crate::types::{position::BlockPos, EntityId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ClientboundBlockBreakingProgressPacket {
    pub entity_id: EntityId,
    pub position: BlockPos,
    pub progress: u8,
}
