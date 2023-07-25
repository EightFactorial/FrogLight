use mc_rs_macros::Transcode;
use hashbrown::HashMap;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSynchronizeTagsS2CPacket {
    pub a: HashMap,
}
