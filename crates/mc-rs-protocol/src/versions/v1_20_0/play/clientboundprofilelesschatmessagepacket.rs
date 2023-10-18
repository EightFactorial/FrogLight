use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundProfilelessChatMessagePacket {
    pub message: String,
    pub chat_type: UnsizedByteBuffer,
}
