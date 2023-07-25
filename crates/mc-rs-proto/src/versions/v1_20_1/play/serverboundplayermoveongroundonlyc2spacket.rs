use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundPlayerMoveOnGroundOnlyC2SPacket {
    pub a: bool,
}
