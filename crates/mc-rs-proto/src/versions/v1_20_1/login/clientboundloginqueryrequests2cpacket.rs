use crate::types::{ResourceLocation, UnsizedByteBuffer};
use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundLoginQueryRequestS2CPacket {
    pub a: u32,
    pub b: ResourceLocation,
    pub c: u32,
    pub d: UnsizedByteBuffer,
}
