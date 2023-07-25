use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBlockEventPacket {
    pub a: BlockPos,
    pub b: u16,
    pub c: u16,
    pub d: Object,
}
