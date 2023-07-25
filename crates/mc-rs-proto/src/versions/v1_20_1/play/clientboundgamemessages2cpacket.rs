use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundGameMessageS2CPacket {
    pub a: FormattedText,
    pub b: bool,
}
