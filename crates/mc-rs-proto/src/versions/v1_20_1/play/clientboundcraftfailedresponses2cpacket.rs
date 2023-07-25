use mc_rs_macros::Packet;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundCraftFailedResponseS2CPacket {
    pub a: u8,
    pub b: ResourceLocation,
}
