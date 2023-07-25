use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundGameMessageS2CPacket {
    pub a: FormattedText,
    pub b: bool,
}
