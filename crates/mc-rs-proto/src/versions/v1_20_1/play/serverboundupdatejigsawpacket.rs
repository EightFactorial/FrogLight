use mc_rs_macros::Transcode;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateJigsawPacket {
    pub a: BlockPos,
    pub b: ResourceLocation,
    pub c: ResourceLocation,
    pub d: ResourceLocation,
    pub e: String,
    pub f: String,
}
