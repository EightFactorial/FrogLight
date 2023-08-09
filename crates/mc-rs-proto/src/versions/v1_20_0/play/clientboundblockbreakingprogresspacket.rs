use mc_rs_macros::Transcode;

use crate::types::{position::BlockPos, EntityId};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBlockBreakingProgressPacket {
    pub entity_id: EntityId,
    pub position: BlockPos,
    pub progress: u8,
}
