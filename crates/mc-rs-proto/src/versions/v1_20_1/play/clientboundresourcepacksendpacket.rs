use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundResourcePackSendPacket {
    pub url: String,
    pub hash: String,
    pub required: bool,
    pub prompt: Option<String>,
}
