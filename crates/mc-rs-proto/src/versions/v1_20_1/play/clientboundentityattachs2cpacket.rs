use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntityAttachS2CPacket {
    pub a: u32,
    pub b: u32,
}
