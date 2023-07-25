use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLoginCompressionS2CPacket {
    #[var]
    pub threshold: i32,
}
