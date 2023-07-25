use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundPlayerMovePositionAndOnGroundC2SPacket {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: bool,
}
