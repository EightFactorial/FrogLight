use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBlockUpdatePacket {
    pub a: BlockPos,
    pub b: Object,
}
