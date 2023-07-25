use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundDeathMessageS2CPacket {
    pub a: u32,
    pub b: FormattedText,
}
