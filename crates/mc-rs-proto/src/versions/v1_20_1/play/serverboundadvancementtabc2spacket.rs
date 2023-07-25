use mc_rs_macros::Packet;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundAdvancementTabC2SPacket {
    pub a: Enum,
    pub b: ResourceLocation,
}
