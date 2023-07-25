use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChatSuggestionsPacket {
    pub a: Enum,
    pub b: Vec,
}
