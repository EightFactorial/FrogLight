use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerInputC2SPacket {
    pub a: f32,
    pub b: f32,
    pub c: u8,
}
