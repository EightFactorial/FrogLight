use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
pub struct ClientboundPlayerListPacket {
    pub actions: u8,
    pub data: UnsizedByteBuffer,
}
