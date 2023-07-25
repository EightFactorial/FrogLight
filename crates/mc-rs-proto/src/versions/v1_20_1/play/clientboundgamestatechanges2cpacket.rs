use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundGameStateChangeS2CPacket {
    pub a: u16,
    pub b: f32,
}
