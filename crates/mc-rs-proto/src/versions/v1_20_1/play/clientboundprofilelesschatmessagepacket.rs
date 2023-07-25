use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundProfilelessChatMessagePacket {
    pub a: FormattedText,
}
