use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerRemoveS2CPacket {
    pub a: Vec,
}
