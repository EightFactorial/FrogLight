use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundBlockUpdateS2CPacket {
    pub a: BlockPos,
    pub b: Object,
}
