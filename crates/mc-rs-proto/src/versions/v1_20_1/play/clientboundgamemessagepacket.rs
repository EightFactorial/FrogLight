use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundGameMessagePacket {
    pub a: FormattedText,
    pub b: bool,
}
