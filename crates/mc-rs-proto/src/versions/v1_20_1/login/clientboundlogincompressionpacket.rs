use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLoginCompressionPacket {
    #[var]
    pub threshold: i32,
}
