use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerListHeaderS2CPacket {
    pub a: FormattedText,
    pub b: FormattedText,
}
