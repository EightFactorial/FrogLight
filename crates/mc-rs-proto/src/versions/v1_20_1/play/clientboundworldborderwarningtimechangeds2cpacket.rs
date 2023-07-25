use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundWorldBorderWarningTimeChangedS2CPacket {
    pub a: u32,
}
