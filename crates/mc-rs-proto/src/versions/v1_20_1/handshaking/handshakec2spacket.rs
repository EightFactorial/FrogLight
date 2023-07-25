use mc_rs_macros::Packet;

#[derive(Debug, Clone, PartialEq, Packet)]
pub struct HandshakeC2SPacket {
    pub a: u32,
    pub b: String,
    pub c: u32,
    pub d: u32,
}
