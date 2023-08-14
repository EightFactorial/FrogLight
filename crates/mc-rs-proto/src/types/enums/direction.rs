use bevy_math::IVec3;
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

impl From<IVec3> for Direction {
    fn from(value: IVec3) -> Self {
        let [x, y, z] = value.into();
        match [x, y, z] {
            [0, -1, 0] => Direction::Down,
            [0, 1, 0] => Direction::Up,
            [0, 0, -1] => Direction::North,
            [0, 0, 1] => Direction::South,
            [-1, 0, 0] => Direction::West,
            [1, 0, 0] => Direction::East,
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<[i32; 3]> for Direction {
    fn from(value: [i32; 3]) -> Self { Self::from(IVec3::from(value)) }
}
