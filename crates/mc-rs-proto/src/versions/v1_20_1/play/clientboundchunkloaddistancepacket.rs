use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkLoadDistancePacket {
    pub a: u32,
}
