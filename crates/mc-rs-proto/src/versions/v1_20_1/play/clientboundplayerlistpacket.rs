use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerListPacket {
    pub actions: u8,
    pub data: UnsizedByteBuffer,
}
