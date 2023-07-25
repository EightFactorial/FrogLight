use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntityMoveRelativeS2CPacket {
    pub a: u32,
    pub b: u16,
    pub c: u16,
    pub d: u16,
    pub e: bool,
}
