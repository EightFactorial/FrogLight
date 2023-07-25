use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundLoginHelloS2CPacket {
    pub a: String,
    pub b: Vec<u8>,
    pub c: Vec<u8>,
}
