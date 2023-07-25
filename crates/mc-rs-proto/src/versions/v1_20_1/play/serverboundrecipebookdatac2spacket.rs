use mc_rs_macros::Packet;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundRecipeBookDataC2SPacket {
    pub a: ResourceLocation,
}
