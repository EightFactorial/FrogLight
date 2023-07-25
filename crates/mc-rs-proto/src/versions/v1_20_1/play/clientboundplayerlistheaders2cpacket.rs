use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundPlayerListHeaderS2CPacket {
    pub a: FormattedText,
    pub b: FormattedText,
}
