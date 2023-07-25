use mc_rs_macros::Packet;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundUpdateJigsawC2SPacket {
    pub a: BlockPos,
    pub b: ResourceLocation,
    pub c: ResourceLocation,
    pub d: ResourceLocation,
    pub e: String,
    pub f: String,
}
