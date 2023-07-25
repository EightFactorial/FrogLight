use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundLightUpdateS2CPacket {
    pub a: u32,
    pub b: u32,
}
