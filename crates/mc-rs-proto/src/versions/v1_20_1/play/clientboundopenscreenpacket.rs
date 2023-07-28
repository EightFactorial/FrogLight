use mc_rs_macros::Transcode;

use crate::types::ResourceLocation;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundOpenScreenPacket {
    #[var]
    pub screen_id: u32,
    pub kind: ResourceLocation,
    pub title: String,
}
