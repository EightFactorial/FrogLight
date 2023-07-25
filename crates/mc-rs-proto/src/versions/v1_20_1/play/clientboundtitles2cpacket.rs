use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundTitleS2CPacket {
    pub a: FormattedText,
}
