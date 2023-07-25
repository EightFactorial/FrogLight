use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayPingS2CPacket {
    pub a: u32,
}
