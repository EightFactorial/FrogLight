use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundUpdateSignC2SPacket {
    pub a: BlockPos,
    pub b: bool,
    pub c: String,
}
