use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 8, 2])]
pub struct ExperienceBarUpdatePacket {
    pub experience: f32,
    #[frog(var)]
    pub level: u32,
    #[frog(var)]
    pub total_experience: u32,
}
