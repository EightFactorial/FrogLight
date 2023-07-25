use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundQueryPongPacket {
    pub time: u64,
}
