use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundProfilelessChatMessageS2CPacket {
    pub a: FormattedText,
}
