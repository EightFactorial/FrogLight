use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundMapUpdatePacket {
    #[var]
    pub map_id: u32,
    pub map_scale: u8,
    pub locked: bool,
    pub data: UnsizedByteBuffer,
}
