use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerMoveOnGroundOnlyC2SPacket {
    pub a: bool,
}
