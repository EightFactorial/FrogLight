use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundKeepAliveS2CPacket {
    pub a: u64,
}
