use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerListS2CPacket {
    pub a: EnumSet,
    pub b: Vec,
}
