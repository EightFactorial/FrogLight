use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundDisconnectPacket {
    pub a: FormattedText,
}
