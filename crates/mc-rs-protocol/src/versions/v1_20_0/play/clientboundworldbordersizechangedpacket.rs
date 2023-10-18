use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderSizeChangedPacket {
    pub size: f64,
}
