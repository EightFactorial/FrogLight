use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct WorldBorderInterpolateSizePacket {
    pub old_diameter: f64,
    pub new_diameter: f64,
    #[frog(var)]
    pub speed: u64,
}
