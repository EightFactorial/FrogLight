use crate::types::UnsizedByteBuffer;
use mc_rs_macros::Transcode;

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundStopSoundPacket {
    pub data: UnsizedByteBuffer,
    // pub flags: u8,
    // pub b: Enum,
    // pub c: ResourceLocation,
}
