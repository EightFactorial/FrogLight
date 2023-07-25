use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundKeepAliveC2SPacket {
    pub a: u64,
}
