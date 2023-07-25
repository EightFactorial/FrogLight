use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundProfilelessChatMessageS2CPacket {
    pub a: FormattedText,
}
