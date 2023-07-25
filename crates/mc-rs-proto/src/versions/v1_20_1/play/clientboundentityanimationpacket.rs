use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundEntityAnimationPacket {
    pub a: u32,
    pub b: u16,
}
