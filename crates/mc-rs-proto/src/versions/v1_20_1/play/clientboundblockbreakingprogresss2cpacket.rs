use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundBlockBreakingProgressS2CPacket {
    pub a: u32,
    pub b: BlockPos,
    pub c: u16,
}
