use compact_str::CompactString;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [3, 5, 77, 67, 45, 82, 83])]
pub enum BossBarAction {
    Add(BossAddAction),
    Remove,
    UpdateProgress(f32),
    UpdateName(CompactString),
    UpdateStyle(BossBarStyle),
    UpdateFlags(BossBarFlags),
}

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct BossAddAction {
    pub name: CompactString,
    pub progress: f32,
    pub style: BossBarStyle,
    pub flags: BossBarFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 1])]
pub struct BossBarStyle {
    pub color: BossBarColor,
    pub division: BossBarDivision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [6])]
pub enum BossBarColor {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [4])]
pub enum BossBarDivision {
    None,
    Notch6,
    Notch10,
    Notch12,
    Notch20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0])]
#[bitset]
pub struct BossBarFlags {
    pub darken_screen: bool,
    pub play_boss_music: bool,
    pub create_fog: bool,
}
