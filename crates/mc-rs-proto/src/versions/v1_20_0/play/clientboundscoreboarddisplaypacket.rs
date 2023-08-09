use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundScoreboardDisplayPacket {
    pub slot: u8,
    pub name: String,
}
