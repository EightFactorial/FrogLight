use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundLoginKeyC2SPacket {
    pub a: Vec<u8>,
    pub b: Vec<u8>,
}
