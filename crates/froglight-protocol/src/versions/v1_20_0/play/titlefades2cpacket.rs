use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2])]
pub struct TitleFadeS2CPacket {
    pub fade_in_ticks: u32,
    pub stay_ticks: u32,
    pub fade_out_ticks: u32,
}
