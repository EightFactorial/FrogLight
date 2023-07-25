use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundFeaturesS2CPacket {
    pub a: Vec,
}
