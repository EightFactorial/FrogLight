use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntityStatusEffectS2CPacket {
    pub a: u32,
    pub b: Object,
    pub c: u8,
    pub d: u32,
    pub e: u8,
    pub f: Object,
}
