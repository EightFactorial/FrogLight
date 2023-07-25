use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundOpenScreenPacket {
    pub a: u32,
    pub b: Object,
    pub c: FormattedText,
}
