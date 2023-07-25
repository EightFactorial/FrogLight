use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkDeltaUpdatePacket {
    pub a: u64,
    pub b: u32,
    pub c: u64,
}
