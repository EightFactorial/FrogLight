use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundOpenScreenS2CPacket {
    pub a: u32,
    pub b: Object,
    pub c: FormattedText,
}
