use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntityStatusS2CPacket {
    pub a: u32,
    pub b: u8,
}
