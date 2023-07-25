use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundScoreboardObjectiveUpdatePacket {
    pub a: String,
    pub b: u8,
    pub c: FormattedText,
    pub d: Enum,
}
