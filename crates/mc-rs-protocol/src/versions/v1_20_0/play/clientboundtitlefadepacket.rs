use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundTitleFadePacket {
    pub fade_in: u32,
    pub stay: u32,
    pub fade_out: u32,
}
