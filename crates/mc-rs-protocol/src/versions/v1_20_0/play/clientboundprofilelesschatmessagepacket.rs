use azalea_chat::FormattedText;
use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundProfilelessChatMessagePacket {
    pub message: FormattedText,
    pub chat_type: UnsizedByteBuffer,
}
