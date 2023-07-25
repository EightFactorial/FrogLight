use mc_rs_macros::Packet;
use hashbrown::HashMap;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundAdvancementUpdateS2CPacket {
    pub a: bool,
    pub b: HashMap,
    pub c: Vec,
    pub d: HashMap,
}
