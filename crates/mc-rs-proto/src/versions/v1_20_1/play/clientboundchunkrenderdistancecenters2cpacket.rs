use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkRenderDistanceCenterS2CPacket {
    pub a: u32,
    pub b: u32,
}
