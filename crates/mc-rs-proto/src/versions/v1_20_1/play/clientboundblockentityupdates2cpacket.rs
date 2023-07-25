use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundBlockEntityUpdateS2CPacket {
    pub a: BlockPos,
    pub b: Object,
    pub c: NbtCompound,
}
