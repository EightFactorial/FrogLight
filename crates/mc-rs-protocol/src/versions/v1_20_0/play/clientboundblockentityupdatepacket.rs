use azalea_nbt::Nbt;
use mc_rs_macros::Transcode;

use crate::types::{position::BlockPos, ResourceLocation};

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ClientboundBlockEntityUpdatePacket {
    pub position: BlockPos,
    pub entity_kind: ResourceLocation,
    pub tag: Nbt,
}
