use mc_rs_macros::Transcode;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundAdvancementTabC2SPacket {
    pub a: Enum,
    pub b: ResourceLocation,
}
