use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundPlayerActionResponseS2CPacket {
    pub a: u32,
}
