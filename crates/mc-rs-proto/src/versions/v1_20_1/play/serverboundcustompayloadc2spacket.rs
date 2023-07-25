use mc_rs_macros::Packet;
use crate::types::ResourceLocation;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundCustomPayloadC2SPacket {
    pub a: ResourceLocation,
    pub b: u32,
    pub c: UnsizedByteBuffer,
}
