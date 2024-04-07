use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct WorldBorderInterpolateSizeS2CPacket {
    pub size: f64,
    pub size_lerp_target: f64,
    #[frog(var)]
    pub size_lerp_time: u64,
}
