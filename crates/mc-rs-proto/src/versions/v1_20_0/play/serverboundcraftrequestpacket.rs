use crate::types::ResourceLocation;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundCraftRequestPacket {
    pub container_id: u8,
    pub recipe: ResourceLocation,
    pub shift: bool,
}
