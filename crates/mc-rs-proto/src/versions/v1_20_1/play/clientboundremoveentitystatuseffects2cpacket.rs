use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundRemoveEntityStatusEffectS2CPacket {
    pub a: u32,
    pub b: Object,
}
