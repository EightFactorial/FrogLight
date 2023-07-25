use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundBoatPaddleStateC2SPacket {
    pub a: bool,
    pub b: bool,
}
