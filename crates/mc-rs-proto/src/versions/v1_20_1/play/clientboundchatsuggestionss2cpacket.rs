use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChatSuggestionsS2CPacket {
    pub a: Enum,
    pub b: Vec,
}
