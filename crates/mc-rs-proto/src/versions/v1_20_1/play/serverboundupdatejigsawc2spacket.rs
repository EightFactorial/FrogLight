use mc_rs_macros::Transcode;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateJigsawC2SPacket {
    pub a: BlockPos,
    pub b: ResourceLocation,
    pub c: ResourceLocation,
    pub d: ResourceLocation,
    pub e: String,
    pub f: String,
}
