use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct HealthUpdatePacket {
    pub health: f32,
    #[frog(var)]
    pub food: u32,
    pub saturation: f32,
}
