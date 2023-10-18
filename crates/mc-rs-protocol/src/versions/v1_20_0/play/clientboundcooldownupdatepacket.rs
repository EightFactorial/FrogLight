use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCooldownUpdatePacket {
    pub item: ResourceLocation,
    #[var]
    pub duration: u32,
}
