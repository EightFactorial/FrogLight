use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundSimulationDistanceS2CPacket {
    pub a: u32,
}
