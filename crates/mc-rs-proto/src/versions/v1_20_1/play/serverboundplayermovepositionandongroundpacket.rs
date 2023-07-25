use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerMovePositionAndOnGroundPacket {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: bool,
}
