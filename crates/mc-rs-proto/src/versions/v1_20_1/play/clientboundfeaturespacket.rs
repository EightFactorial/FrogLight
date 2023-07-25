use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundFeaturesPacket {
    pub a: Vec,
}
