use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundSubtitleS2CPacket {
    pub a: FormattedText,
}
