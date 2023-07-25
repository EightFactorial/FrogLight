use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundEntityRotateS2CPacket {
    pub a: u32,
    pub b: u8,
    pub c: u8,
    pub d: bool,
}
