use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundPlayerRemoveS2CPacket {
    pub a: Vec,
}
