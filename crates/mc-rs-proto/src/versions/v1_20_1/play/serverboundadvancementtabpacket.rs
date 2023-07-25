use mc_rs_macros::Transcode;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundAdvancementTabPacket {
    pub a: Enum,
    pub b: ResourceLocation,
}
