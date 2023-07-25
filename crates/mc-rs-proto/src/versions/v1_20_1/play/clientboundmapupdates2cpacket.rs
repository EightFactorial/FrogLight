use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundMapUpdateS2CPacket {
    pub a: u32,
    pub b: u8,
    pub c: bool,
    pub d: Object,
    pub e: u16,
    pub f: u16,
    pub g: u16,
    pub h: u16,
    pub i: Vec<u8>,
}
