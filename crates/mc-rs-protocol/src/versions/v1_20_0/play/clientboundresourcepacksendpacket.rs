use azalea_chat::FormattedText;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundResourcePackSendPacket {
    pub url: String,
    pub hash: String,
    pub required: bool,
    pub prompt: Option<FormattedText>,
}
