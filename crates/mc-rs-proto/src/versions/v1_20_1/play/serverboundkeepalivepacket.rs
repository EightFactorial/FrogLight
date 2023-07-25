use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundKeepAlivePacket {
    pub a: u64,
}
