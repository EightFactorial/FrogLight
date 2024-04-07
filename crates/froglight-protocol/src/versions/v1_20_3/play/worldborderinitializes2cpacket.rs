use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct WorldBorderInitializeS2CPacket {
    pub center_x: (),
    pub center_z: (),
    pub size: (),
    pub size_lerp_target: (),
    pub size_lerp_time: (),
    pub max_radius: (),
    pub warning_blocks: (),
    pub warning_time: (),
}
