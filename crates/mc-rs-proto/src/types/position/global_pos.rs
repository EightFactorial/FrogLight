use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

use super::BlockPos;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Transcode)]
pub struct GlobalPos {
    pub position: BlockPos,
    pub dimension: ResourceLocation,
}