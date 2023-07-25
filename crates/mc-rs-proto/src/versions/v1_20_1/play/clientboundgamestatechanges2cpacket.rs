use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundGameStateChangeS2CPacket {
    pub a: u16,
    pub b: f32,
}
