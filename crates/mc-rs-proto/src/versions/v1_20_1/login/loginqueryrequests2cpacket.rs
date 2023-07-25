use mc_rs_macros::Packet;

use crate::types::{ResourceLocation, UnsizedByteBuffer};

#[derive(Debug, Clone, Packet)]
pub struct LoginQueryRequestS2CPacket {
    pub a: u32,
    pub b: ResourceLocation,
    pub c: u32,
    pub d: UnsizedByteBuffer,
}
