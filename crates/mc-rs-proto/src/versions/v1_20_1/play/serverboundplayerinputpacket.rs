use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPlayerInputPacket {
    pub a: f32,
    pub b: f32,
    pub c: u8,
}
