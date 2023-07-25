use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundExperienceBarUpdateS2CPacket {
    pub a: f32,
    pub b: u32,
    pub c: u32,
}
