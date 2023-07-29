use fastnbt::Value;
use mc_rs_macros::Transcode;

use crate::types::{position::BlockPos, ResourceLocation};

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBlockEntityUpdatePacket {
    pub position: BlockPos,
    pub entity_kind: ResourceLocation,
    pub tag: Value,
}
