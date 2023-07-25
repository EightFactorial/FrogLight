use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundExperienceBarUpdatePacket {
    pub a: f32,
    pub b: u32,
    pub c: u32,
}
