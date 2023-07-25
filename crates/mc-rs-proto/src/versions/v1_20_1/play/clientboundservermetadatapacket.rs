use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundServerMetadataPacket {
    pub a: FormattedText,
    pub b: Option,
    pub c: bool,
}
