use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 8, 2])]
pub struct ClientboundExperienceBarUpdatePacket {
    pub progress: f32,
    #[var]
    pub level: u32,
    #[var]
    pub total_xp: u32,
}
