use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLoginDisconnectS2CPacket {
    pub reason: String,
}
