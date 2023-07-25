use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundWorldEventS2CPacket {
    pub a: u32,
    pub b: BlockPos,
    pub c: u32,
    pub d: bool,
}
