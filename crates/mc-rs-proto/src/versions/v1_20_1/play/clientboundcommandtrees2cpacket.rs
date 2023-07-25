use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundCommandTreeS2CPacket {
    pub a: Vec,
    pub b: u32,
}
