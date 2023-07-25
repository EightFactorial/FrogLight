use mc_rs_macros::Packet;
use hashbrown::HashMap;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundSynchronizeTagsS2CPacket {
    pub a: HashMap,
}
