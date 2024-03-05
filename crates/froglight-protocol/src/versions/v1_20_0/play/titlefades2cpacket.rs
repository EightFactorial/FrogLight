use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct TitleFadeS2CPacket {
    pub fade_in_ticks: (),
    pub stay_ticks: (),
    pub fade_out_ticks: (),
}
