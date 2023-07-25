use mc_rs_macros::Packet;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundCraftRequestC2SPacket {
    pub a: u8,
    pub b: ResourceLocation,
    pub c: bool,
}
