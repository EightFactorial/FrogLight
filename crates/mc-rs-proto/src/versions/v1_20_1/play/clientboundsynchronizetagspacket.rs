use mc_rs_macros::Transcode;
use hashbrown::HashMap;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSynchronizeTagsPacket {
    pub a: HashMap,
}
