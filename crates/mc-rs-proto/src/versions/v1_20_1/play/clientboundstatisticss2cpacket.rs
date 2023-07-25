use mc_rs_macros::Packet;
use hashbrown::HashMap;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundStatisticsS2CPacket {
    pub a: HashMap,
}
