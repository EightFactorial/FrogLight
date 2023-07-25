use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundPlayerMoveLookAndOnGroundC2SPacket {
    pub a: f32,
    pub b: f32,
    pub c: bool,
}
