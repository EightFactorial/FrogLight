use mc_rs_macros::Transcode;

use crate::types::{ResourceLocation, UnsizedByteBuffer};

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundLoginQueryResponseC2SPacket {
    #[var]
    pub id: u32,
    pub identifier: ResourceLocation,
    pub data: UnsizedByteBuffer,
}
