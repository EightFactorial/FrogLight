use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundScoreboardPlayerUpdateS2CPacket {
    pub a: String,
    pub b: Enum,
    pub c: String,
    pub d: u32,
}
