use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundDamageTiltPacket {
    pub a: u32,
    pub b: f32,
}
