use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundScoreboardDisplayPacket {
    pub a: u8,
    pub b: String,
}
