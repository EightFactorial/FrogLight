use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundExperienceBarUpdatePacket {
    pub progress: f32,
    #[var]
    pub level: u32,
    #[var]
    pub total_xp: u32,
}
