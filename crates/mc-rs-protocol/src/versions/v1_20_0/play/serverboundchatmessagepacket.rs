use compact_str::CompactString;
use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

// TODO: Decode this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
pub struct ServerboundChatMessagePacket {
    pub message: CompactString,
    pub timestamp: u64,
    pub salt: u64,
    pub data: UnsizedByteBuffer,
}
