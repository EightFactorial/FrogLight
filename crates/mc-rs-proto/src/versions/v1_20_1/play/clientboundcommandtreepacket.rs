use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundCommandTreePacket {
    // TODO: Parse data
    pub data: UnsizedByteBuffer,
}
