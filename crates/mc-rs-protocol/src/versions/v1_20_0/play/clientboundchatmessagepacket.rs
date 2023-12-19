use mc_rs_macros::Transcode;

use crate::types::{EntityUuid, UnsizedByteBuffer};

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChatMessagePacket {
    pub sender: EntityUuid,
    #[var]
    pub index: u32,
    pub data: UnsizedByteBuffer,
}
