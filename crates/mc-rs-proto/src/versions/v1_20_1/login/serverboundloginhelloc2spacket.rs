use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundLoginHelloC2SPacket {
    pub a: String,
    // pub b: Option,
}
