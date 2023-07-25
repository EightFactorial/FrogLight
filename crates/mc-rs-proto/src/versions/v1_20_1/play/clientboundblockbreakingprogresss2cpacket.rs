use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBlockBreakingProgressS2CPacket {
    pub a: u32,
    pub b: BlockPos,
    pub c: u16,
}
