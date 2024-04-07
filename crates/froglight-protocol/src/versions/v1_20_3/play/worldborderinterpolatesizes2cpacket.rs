use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct WorldBorderInterpolateSizeS2CPacket {
    pub size: (),
    pub size_lerp_target: (),
    pub size_lerp_time: (),
}
