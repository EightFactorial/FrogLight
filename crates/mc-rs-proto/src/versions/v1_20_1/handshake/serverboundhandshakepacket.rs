use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundHandshakePacket {
    #[var]
    pub protocol_version: i32,
    pub hostname: String,
    pub port: u16,
    #[var]
    pub intention: i32,
}
