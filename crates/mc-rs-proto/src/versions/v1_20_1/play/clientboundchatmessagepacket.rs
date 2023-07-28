use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChatMessagePacket {
    pub sender: Uuid,
    #[var]
    pub index: u32,
    pub data: UnsizedByteBuffer,
}
