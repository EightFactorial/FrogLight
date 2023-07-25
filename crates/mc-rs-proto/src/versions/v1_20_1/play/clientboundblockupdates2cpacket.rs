use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBlockUpdateS2CPacket {
    pub a: BlockPos,
    pub b: Object,
}
