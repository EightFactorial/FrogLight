use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldTimeUpdatePacket {
    pub a: u64,
    pub b: u64,
}
