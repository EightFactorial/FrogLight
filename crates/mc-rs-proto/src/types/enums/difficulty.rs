use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}
