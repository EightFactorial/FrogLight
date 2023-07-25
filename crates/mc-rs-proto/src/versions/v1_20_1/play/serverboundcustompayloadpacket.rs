use mc_rs_macros::Transcode;
use crate::types::ResourceLocation;
use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundCustomPayloadPacket {
    pub a: ResourceLocation,
    pub b: u32,
    pub c: UnsizedByteBuffer,
}
