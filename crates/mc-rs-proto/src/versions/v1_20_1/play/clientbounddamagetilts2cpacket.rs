use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundDamageTiltS2CPacket {
    pub a: u32,
    pub b: f32,
}
