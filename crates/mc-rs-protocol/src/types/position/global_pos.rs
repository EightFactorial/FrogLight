use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

use super::BlockPos;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct GlobalPos {
    pub world: ResourceLocation,
    pub position: BlockPos,
}
