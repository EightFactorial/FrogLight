use mc_rs_macros::Transcode;

use crate::types::packets::tags::TagMap;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSynchronizeTagsPacket {
    pub tags: TagMap,
}
