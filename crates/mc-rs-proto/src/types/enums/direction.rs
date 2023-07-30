use mc_rs_macros::Transcode;
use strum::{Display, EnumString};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, Transcode)]
pub enum Direction {
    #[default]
    Down,
    Up,
    North,
    South,
    West,
    East,
}
