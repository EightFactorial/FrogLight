use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerAbilitiesPacket {
    pub a: u8,
    pub b: f32,
    pub c: f32,
}
