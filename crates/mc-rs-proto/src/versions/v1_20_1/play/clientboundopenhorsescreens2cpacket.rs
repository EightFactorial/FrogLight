use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundOpenHorseScreenS2CPacket {
    pub a: u16,
    pub b: u32,
    pub c: u32,
}
