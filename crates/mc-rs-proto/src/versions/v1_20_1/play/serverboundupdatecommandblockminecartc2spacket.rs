use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundUpdateCommandBlockMinecartC2SPacket {
    pub a: u32,
    pub b: String,
    pub c: bool,
}
