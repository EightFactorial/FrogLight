use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundHandshakeC2SPacket {
    pub a: u32,
    pub b: String,
    pub c: u32,
    pub d: u32,
}
