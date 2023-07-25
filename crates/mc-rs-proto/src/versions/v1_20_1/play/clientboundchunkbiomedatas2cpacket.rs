use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChunkBiomeDataS2CPacket {
    pub a: Vec,
}
