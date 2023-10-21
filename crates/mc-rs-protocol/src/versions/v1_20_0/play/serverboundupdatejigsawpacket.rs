use crate::types::{position::BlockPos, ResourceLocation};
use compact_str::CompactString;
use mc_rs_macros::Transcode;

// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
pub struct ServerboundUpdateJigsawPacket {
    pub position: BlockPos,
    pub name: ResourceLocation,
    pub target: ResourceLocation,
    pub pool: ResourceLocation,
    pub final_state: CompactString,
    pub joint: CompactString,
}
