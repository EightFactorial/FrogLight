use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundDifficultyPacket {
    pub a: u16,
    pub b: bool,
}
