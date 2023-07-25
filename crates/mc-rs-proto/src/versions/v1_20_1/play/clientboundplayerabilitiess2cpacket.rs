use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundPlayerAbilitiesS2CPacket {
    pub a: u8,
    pub b: f32,
    pub c: f32,
}
