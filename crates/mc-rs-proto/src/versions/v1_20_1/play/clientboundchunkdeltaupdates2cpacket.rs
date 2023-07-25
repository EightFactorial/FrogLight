use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ClientboundChunkDeltaUpdateS2CPacket {
    pub a: u64,
    pub b: u32,
    pub c: u64,
}
