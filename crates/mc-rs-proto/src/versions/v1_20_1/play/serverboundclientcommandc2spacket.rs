use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundClientCommandC2SPacket {
    pub a: u32,
    pub b: Enum,
    pub c: u32,
}
