use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundServerMetadataS2CPacket {
    pub a: FormattedText,
    pub b: Option,
    pub c: bool,
}
