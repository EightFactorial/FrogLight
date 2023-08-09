use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundServerMetadataPacket {
    pub motd: String,
    pub icon: Option<Vec<u8>>,
    pub enforce_secure_chat: bool,
}
