use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundPlayerListHeaderPacket {
    pub a: FormattedText,
    pub b: FormattedText,
}
