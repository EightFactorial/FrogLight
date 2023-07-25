use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerListPacket {
    pub a: EnumSet,
    pub b: Vec,
}
