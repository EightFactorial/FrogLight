use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundWorldBorderSizeChangedS2CPacket {
    pub a: f64,
}
