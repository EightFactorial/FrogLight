use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntityDamageS2CPacket {
    pub a: u32,
    pub b: u32,
    pub c: Option,
}
