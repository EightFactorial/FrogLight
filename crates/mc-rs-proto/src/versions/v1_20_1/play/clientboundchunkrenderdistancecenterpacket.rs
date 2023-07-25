use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkRenderDistanceCenterPacket {
    pub a: u32,
    pub b: u32,
}
