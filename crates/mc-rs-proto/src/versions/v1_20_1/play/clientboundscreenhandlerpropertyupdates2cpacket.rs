use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundScreenHandlerPropertyUpdateS2CPacket {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}
