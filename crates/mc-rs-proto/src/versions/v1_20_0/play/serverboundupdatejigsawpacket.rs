use crate::types::{position::BlockPos, ResourceLocation};
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateJigsawPacket {
    pub position: BlockPos,
    pub name: ResourceLocation,
    pub target: ResourceLocation,
    pub pool: ResourceLocation,
    pub final_state: String,
    pub joint: String,
}
