use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundDifficultyS2CPacket {
    pub a: u16,
    pub b: bool,
}
