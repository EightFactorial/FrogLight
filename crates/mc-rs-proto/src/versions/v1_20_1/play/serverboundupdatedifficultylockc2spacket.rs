use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateDifficultyLockC2SPacket {
    pub a: bool,
}
