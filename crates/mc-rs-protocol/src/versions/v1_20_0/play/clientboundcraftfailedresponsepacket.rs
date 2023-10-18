use crate::types::ResourceLocation;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCraftFailedResponsePacket {
    pub container_id: u8,
    pub recipe: ResourceLocation,
}
