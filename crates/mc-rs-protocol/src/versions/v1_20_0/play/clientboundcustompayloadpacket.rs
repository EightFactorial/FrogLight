use crate::types::{ResourceLocation, UnsizedByteBuffer};
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: UnsizedByteBuffer,
}
