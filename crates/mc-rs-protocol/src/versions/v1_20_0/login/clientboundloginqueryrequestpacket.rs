use crate::types::UnsizedByteBuffer;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 1, 1, 2, 3, 4])]
pub struct ClientboundLoginQueryRequestPacket {
    #[var]
    pub id: u32,
    pub data: Option<UnsizedByteBuffer>,
}
