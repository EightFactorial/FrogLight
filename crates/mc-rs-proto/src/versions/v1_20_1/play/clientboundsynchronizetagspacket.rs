use mc_rs_macros::Transcode;

use crate::types::UnsizedByteBuffer;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSynchronizeTagsPacket {
    pub tags: UnsizedByteBuffer,
}
