use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundQueryPingC2SPacket {
    pub time: u64,
}
