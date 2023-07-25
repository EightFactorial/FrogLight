use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateDifficultyC2SPacket {
    pub a: u16,
}
