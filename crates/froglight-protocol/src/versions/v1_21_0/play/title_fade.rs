use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2])]
pub struct TitleFadePacket {
    pub fade_in_ticks: u32,
    pub stay_ticks: u32,
    pub fade_out_ticks: u32,
}
