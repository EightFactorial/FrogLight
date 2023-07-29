use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundTeamPacket {
    pub name: String,
    pub method: UnsizedByteBuffer,
}
