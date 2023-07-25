use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntityRotateAndMoveRelativeS2CPacket {
    pub a: u32,
    pub b: u16,
    pub c: u16,
    pub d: u16,
    pub e: u8,
    pub f: u8,
    pub g: bool,
}
