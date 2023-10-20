use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCommandSuggestionsPacket {
    #[var]
    pub id: u32,
    pub data: UnsizedByteBuffer,
}
