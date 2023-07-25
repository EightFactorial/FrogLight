use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundGameStateChangePacket {
    pub a: u16,
    pub b: f32,
}
