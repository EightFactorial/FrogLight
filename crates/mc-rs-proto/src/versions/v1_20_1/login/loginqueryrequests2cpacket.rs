use crate::types::{ResourceLocation, UnsizedByteBuffer};

#[derive(Debug, Clone)]
pub struct LoginQueryRequestS2CPacket {
    pub a: u32,
    pub b: ResourceLocation,
    pub c: u32,
    pub d: UnsizedByteBuffer,
}
