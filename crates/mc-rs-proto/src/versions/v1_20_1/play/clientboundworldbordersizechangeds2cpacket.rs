use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderSizeChangedS2CPacket {
    pub a: f64,
}
