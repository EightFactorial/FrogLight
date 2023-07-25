use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundScoreboardObjectiveUpdateS2CPacket {
    pub a: String,
    pub b: u8,
    pub c: FormattedText,
    pub d: Enum,
}
