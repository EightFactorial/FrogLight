use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundSelectAdvancementTabS2CPacket {
    pub a: Object,
}
