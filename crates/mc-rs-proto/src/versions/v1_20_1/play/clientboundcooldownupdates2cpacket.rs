use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundCooldownUpdateS2CPacket {
    pub a: Object,
    pub b: u32,
}
