use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundServerMetadataS2CPacket {
    pub a: FormattedText,
    pub b: Option,
    pub c: bool,
}
