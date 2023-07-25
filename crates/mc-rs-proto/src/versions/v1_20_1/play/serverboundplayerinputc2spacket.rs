use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundPlayerInputC2SPacket {
    pub a: f32,
    pub b: f32,
    pub c: u8,
}
