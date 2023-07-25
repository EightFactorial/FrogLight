use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateDifficultyLockPacket {
    pub a: bool,
}
