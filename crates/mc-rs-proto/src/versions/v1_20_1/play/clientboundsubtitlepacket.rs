use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSubtitlePacket {
    pub text: String,
}
