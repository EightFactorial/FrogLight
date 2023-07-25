use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundResourcePackSendS2CPacket {
    pub a: String,
    pub b: String,
    pub c: bool,
    pub d: Object,
}
