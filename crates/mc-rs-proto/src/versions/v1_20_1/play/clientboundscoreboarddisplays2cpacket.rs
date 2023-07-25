use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundScoreboardDisplayS2CPacket {
    pub a: u8,
    pub b: String,
}
