use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldTimeUpdateS2CPacket {
    pub a: u64,
    pub b: u64,
}
