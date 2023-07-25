use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBlockEntityUpdatePacket {
    pub a: BlockPos,
    pub b: Object,
    pub c: NbtCompound,
}
