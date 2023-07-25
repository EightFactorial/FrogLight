use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBlockEntityUpdateS2CPacket {
    pub a: BlockPos,
    pub b: Object,
    pub c: NbtCompound,
}
