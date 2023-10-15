use bevy_math::IVec3;
use mc_rs_macros::Transcode;
use strum::{Display, EnumString};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, Transcode)]
#[test(transcode)]
pub enum Direction {
    #[default]
    Up,
    Down,
    North,
    South,
    East,
    West,
}

impl From<IVec3> for Direction {
    fn from(value: IVec3) -> Self {
        let [x, y, z] = value.into();
        match [x, y, z] {
            [0, 1, 0] => Direction::Up,
            [0, -1, 0] => Direction::Down,
            [0, 0, -1] => Direction::North,
            [0, 0, 1] => Direction::South,
            [1, 0, 0] => Direction::East,
            [-1, 0, 0] => Direction::West,
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<Direction> for IVec3 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => IVec3::new(0, 1, 0),
            Direction::Down => IVec3::new(0, -1, 0),
            Direction::North => IVec3::new(0, 0, -1),
            Direction::South => IVec3::new(0, 0, 1),
            Direction::East => IVec3::new(1, 0, 0),
            Direction::West => IVec3::new(-1, 0, 0),
        }
    }
}

impl From<[i32; 3]> for Direction {
    fn from(value: [i32; 3]) -> Self { Self::from(IVec3::from(value)) }
}

impl From<(i32, i32, i32)> for Direction {
    fn from(value: (i32, i32, i32)) -> Self { Self::from(IVec3::from(value)) }
}
