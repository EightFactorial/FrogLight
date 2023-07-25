use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundPlayerPositionLookS2CPacket {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f32,
    pub e: f32,
    pub f: u16,
    pub g: u32,
}
