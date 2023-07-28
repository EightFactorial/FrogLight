use mc_rs_macros::{Encode, Transcode};

#[derive(Debug, Clone, PartialEq, Encode)]
pub enum BossBarAction {
    Add(BossAddAction),
    Remove,
    UpdateProgress(f32),
    UpdateName(String),
    UpdateStyle(BossBarStyle),
    UpdateFlags(BossBarFlags),
}

#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct BossAddAction {
    pub name: String,
    pub progress: f32,
    pub style: BossBarStyle,
    pub flags: BossBarFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
pub struct BossBarStyle {
    pub color: BossBarColor,
    pub division: BossBarDivision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
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
pub enum BossBarDivision {
    None,
    Notch6,
    Notch10,
    Notch12,
    Notch20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
pub struct BossBarFlags {
    pub darken_screen: bool,
    pub play_boss_music: bool,
    pub create_fog: bool,
}
