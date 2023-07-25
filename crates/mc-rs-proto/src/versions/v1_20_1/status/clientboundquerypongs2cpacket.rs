use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundQueryPongS2CPacket {
    pub time: u64,
}
