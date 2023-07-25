use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundScoreboardPlayerUpdatePacket {
    pub a: String,
    pub b: Enum,
    pub c: String,
    pub d: u32,
}
