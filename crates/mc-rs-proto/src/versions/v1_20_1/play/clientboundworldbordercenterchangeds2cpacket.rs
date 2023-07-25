use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderCenterChangedS2CPacket {
    pub a: f64,
    pub b: f64,
}
