use mc_rs_macros::Transcode;
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, Transcode)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}
