use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCommandSuggestionsPacket {
    #[var]
    pub id: u32,
    // TODO: Parse data
    pub data: UnsizedByteBuffer,
}
