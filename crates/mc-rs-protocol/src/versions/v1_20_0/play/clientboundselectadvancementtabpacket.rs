use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSelectAdvancementTabPacket {
    pub tab: Option<ResourceLocation>,
}
