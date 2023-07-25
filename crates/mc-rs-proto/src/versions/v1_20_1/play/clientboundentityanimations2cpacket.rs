use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntityAnimationS2CPacket {
    pub a: u32,
    pub b: u16,
}
