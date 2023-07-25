use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundDifficultyS2CPacket {
    pub a: u16,
    pub b: bool,
}
