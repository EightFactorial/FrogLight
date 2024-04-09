use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 8, 2])]
pub struct ExperienceBarUpdateS2CPacket {
    pub bar_progress: f32,
    #[frog(var)]
    pub experience: u32,
    #[frog(var)]
    pub experience_level: u32,
}
