use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderInterpolateSizeS2CPacket {
    pub a: f64,
    pub b: f64,
    pub c: u64,
}
