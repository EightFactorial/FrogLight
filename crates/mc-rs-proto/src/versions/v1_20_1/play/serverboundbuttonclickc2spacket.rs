use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundButtonClickC2SPacket {
    pub a: u8,
    pub b: u8,
}
