use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerActionResponseS2CPacket {
    pub a: u32,
}
