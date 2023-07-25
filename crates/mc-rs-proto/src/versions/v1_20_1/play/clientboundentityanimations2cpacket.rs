use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityAnimationS2CPacket {
    pub a: u32,
    pub b: u16,
}
