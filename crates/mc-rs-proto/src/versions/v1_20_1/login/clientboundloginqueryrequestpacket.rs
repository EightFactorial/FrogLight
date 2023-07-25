use crate::types::UnsizedByteBuffer;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLoginQueryRequestPacket {
    #[var]
    pub id: u32,
    pub data: Option<UnsizedByteBuffer>,
}
