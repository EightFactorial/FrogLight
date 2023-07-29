use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerListHeaderPacket {
    pub header: String,
    pub footer: String,
}
