use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkDeltaUpdateS2CPacket {
    pub a: u64,
    pub b: u32,
    pub c: u64,
}
