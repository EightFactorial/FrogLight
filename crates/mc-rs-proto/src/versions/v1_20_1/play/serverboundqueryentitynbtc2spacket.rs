use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundQueryEntityNbtC2SPacket {
    pub a: u32,
    pub b: u32,
}
