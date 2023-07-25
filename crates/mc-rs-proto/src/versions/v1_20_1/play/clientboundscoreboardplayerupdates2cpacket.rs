use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundScoreboardPlayerUpdateS2CPacket {
    pub a: String,
    pub b: Enum,
    pub c: String,
    pub d: u32,
}
