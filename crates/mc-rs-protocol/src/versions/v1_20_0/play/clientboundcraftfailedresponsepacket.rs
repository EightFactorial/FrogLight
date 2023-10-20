use crate::types::ResourceLocation;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0])]
pub struct ClientboundCraftFailedResponsePacket {
    pub container_id: u8,
    pub recipe: ResourceLocation,
}
