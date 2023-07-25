use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundJigsawGeneratingC2SPacket {
    pub a: BlockPos,
    pub b: u32,
    pub c: bool,
}
