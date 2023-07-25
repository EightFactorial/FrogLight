use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundDisconnectS2CPacket {
    pub a: FormattedText,
}
