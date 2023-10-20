use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::types::UnsizedByteBuffer;

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChatMessagePacket {
    pub sender: Uuid,
    #[var]
    pub index: u32,
    pub data: UnsizedByteBuffer,
}
