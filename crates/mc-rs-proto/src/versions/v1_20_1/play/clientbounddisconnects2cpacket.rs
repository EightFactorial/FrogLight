use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundDisconnectS2CPacket {
    pub a: FormattedText,
}
