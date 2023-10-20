use mc_rs_macros::Transcode;

use crate::types::{ResourceLocation, UnsizedByteBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 15, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 98, 114, 97, 110, 100, 1, 2, 3, 4])]
pub struct ServerboundLoginQueryResponsePacket {
    #[var]
    pub id: u32,
    pub identifier: ResourceLocation,
    pub data: UnsizedByteBuffer,
}
