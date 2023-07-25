use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundKeepAlivePacket {
    pub a: u64,
}
