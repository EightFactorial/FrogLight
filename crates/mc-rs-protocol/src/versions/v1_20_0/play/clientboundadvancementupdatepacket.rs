use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, Transcode)]
pub struct ClientboundAdvancementUpdatePacket {
    pub reset: bool,
    pub data: UnsizedByteBuffer,
    // pub added: HashMap<ResourceLocation, Advancement>,
    // pub removed: Vec<ResourceLocation>,
    // pub progress: HashMap<ResourceLocation, AdvancementProgress>,
}
