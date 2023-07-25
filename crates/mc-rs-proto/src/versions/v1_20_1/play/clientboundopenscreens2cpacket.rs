use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundOpenScreenS2CPacket {
    pub a: u32,
    pub b: Object,
    pub c: FormattedText,
}
