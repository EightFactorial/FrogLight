use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundMapUpdatePacket {
    #[var]
    pub map_id: u32,
    pub map_scale: u8,
    pub locked: bool,
    pub data: UnsizedByteBuffer,
}
