use std::{fmt::Display, str::FromStr};

use glam::IVec3;
// use froglight_macros::FrogReadWrite;

/// A direction in 3D space.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
// #[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub enum Direction {
    #[default]
    /// Facing +Y
    Up,
    /// Facing -Y
    Down,
    /// Facing -Z, 180 degrees
    North,
    /// Facing +X, 0 degrees
    South,
    /// Facing +X, -90 degrees
    East,
    /// Facing -X, 90 degrees
    West,
}

impl Direction {
    /// A [`IVec3`] pointing in the [`Direction::Up`] direction.
    pub const UP: IVec3 = IVec3::new(0, 1, 0);
    /// A [`IVec3`] pointing in the [`Direction::Down`] direction.
    pub const DOWN: IVec3 = IVec3::new(0, -1, 0);
    /// A [`IVec3`] pointing in the [`Direction::North`] direction.
    pub const NORTH: IVec3 = IVec3::new(0, 0, -1);
    /// A [`IVec3`] pointing in the [`Direction::South`] direction.
    pub const SOUTH: IVec3 = IVec3::new(0, 0, 1);
    /// A [`IVec3`] pointing in the [`Direction::East`] direction.
    pub const EAST: IVec3 = IVec3::new(1, 0, 0);
    /// A [`IVec3`] pointing in the [`Direction::West`] direction.
    pub const WEST: IVec3 = IVec3::new(-1, 0, 0);

    /// Returns the opposite direction.
    #[must_use]
    #[inline]
    pub const fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    /// Converts a [`Direction`] into an [`IVec3`].
    #[must_use]
    #[inline]
    pub fn to_axis(&self) -> IVec3 { IVec3::from(*self) }

    /// Attempts to convert an [`IVec3`] into a [`Direction`].
    #[must_use]
    #[inline]
    pub fn try_from_axis(axis: IVec3) -> Option<Direction> { Self::try_from(axis).ok() }
}

impl TryFrom<IVec3> for Direction {
    type Error = ();
    #[inline]
    fn try_from(value: IVec3) -> Result<Self, Self::Error> {
        match value {
            Direction::UP => Ok(Direction::Up),
            Direction::DOWN => Ok(Direction::Down),
            Direction::NORTH => Ok(Direction::North),
            Direction::SOUTH => Ok(Direction::South),
            Direction::EAST => Ok(Direction::East),
            Direction::WEST => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

impl From<Direction> for IVec3 {
    #[inline]
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Direction::UP,
            Direction::Down => Direction::DOWN,
            Direction::North => Direction::NORTH,
            Direction::South => Direction::SOUTH,
            Direction::East => Direction::EAST,
            Direction::West => Direction::WEST,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "north" => Ok(Direction::North),
            "south" => Ok(Direction::South),
            "east" => Ok(Direction::East),
            "west" => Ok(Direction::West),
            _ => Err(()),
        }
    }
}
