use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundFeaturesS2CPacket {
    pub a: Vec,
}
