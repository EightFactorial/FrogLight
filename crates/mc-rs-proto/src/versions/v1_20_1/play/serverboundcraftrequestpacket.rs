use mc_rs_macros::Transcode;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundCraftRequestPacket {
    pub a: u8,
    pub b: ResourceLocation,
    pub c: bool,
}
