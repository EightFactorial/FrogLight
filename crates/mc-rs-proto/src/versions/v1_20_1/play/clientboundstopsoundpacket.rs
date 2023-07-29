use crate::types::UnsizedByteBuffer;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundStopSoundPacket {
    pub data: UnsizedByteBuffer,
    // pub flags: u8,
    // pub b: Enum,
    // pub c: ResourceLocation,
}
