use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntityPassengersSetS2CPacket {
    pub a: u32,
    pub b: Vec<u32>,
}
