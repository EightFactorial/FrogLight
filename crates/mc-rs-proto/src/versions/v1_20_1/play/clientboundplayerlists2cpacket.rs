use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundPlayerListS2CPacket {
    pub a: EnumSet,
    pub b: Vec,
}
