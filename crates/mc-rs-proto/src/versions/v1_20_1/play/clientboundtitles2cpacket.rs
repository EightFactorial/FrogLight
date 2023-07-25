use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundTitleS2CPacket {
    pub a: FormattedText,
}
