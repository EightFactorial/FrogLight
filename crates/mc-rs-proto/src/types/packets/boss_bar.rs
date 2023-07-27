#[derive(Debug, Clone, PartialEq)]
pub enum BossBarAction {
    Add(BossAddAction),
    Remove,
    UpdateProgress(f32),
    UpdateName(String),
    UpdateStyle(BossBarStyle),
    UpdateFlags(BossBarFlags),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BossAddAction {
    pub name: String,
    pub progress: f32,
    pub style: BossBarStyle,
    pub flags: BossBarFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BossBarStyle {
    pub color: BossBarColor,
    pub division: BossBarDivision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossBarColor {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossBarDivision {
    None,
    Notch6,
    Notch10,
    Notch12,
    Notch20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BossBarFlags {
    pub darken_screen: bool,
    pub play_boss_music: bool,
    pub create_fog: bool,
}
