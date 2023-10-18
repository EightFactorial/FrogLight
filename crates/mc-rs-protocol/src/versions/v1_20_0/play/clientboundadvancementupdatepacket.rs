use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundAdvancementUpdatePacket {
    pub reset: bool,
    pub data: UnsizedByteBuffer,
    // pub added: HashMap<ResourceLocation, Advancement>,
    // pub removed: Vec<ResourceLocation>,
    // pub progress: HashMap<ResourceLocation, AdvancementProgress>,
}
