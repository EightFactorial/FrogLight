use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundOverlayMessageS2CPacket {
    pub a: FormattedText,
}
