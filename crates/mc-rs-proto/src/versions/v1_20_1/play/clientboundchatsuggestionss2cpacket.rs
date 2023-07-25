use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundChatSuggestionsS2CPacket {
    pub a: Enum,
    pub b: Vec,
}
