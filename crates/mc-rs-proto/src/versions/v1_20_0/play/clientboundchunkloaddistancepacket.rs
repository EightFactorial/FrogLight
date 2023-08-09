use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkLoadDistancePacket {
    #[var]
    pub distance: u32,
}
