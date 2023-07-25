use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundTeamS2CPacket {
    pub a: String,
    pub b: u8,
    pub c: Vec,
}
