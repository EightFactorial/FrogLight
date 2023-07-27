use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

use super::BlockPos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Transcode)]
pub struct GlobalPos {
    pub position: BlockPos,
    pub dimension: ResourceLocation,
}
