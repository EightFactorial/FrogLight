use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundQueryPingPacket {
    pub time: u64,
}
