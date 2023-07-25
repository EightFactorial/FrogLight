use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundTitleFadeS2CPacket {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}
