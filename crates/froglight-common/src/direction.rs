use std::{fmt::Display, str::FromStr};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::ReflectDefault;
#[cfg(all(feature = "bevy", feature = "serde"))]
use bevy_reflect::prelude::{ReflectDeserialize, ReflectSerialize};
use glam::IVec3;

/// A direction in 3D space.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Default))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub enum Direction {
    /// Facing +Y
    #[default]
    #[cfg_attr(feature = "serde", serde(alias = "top"))]
    Up,
    /// Facing -Y
    #[cfg_attr(feature = "serde", serde(alias = "bottom"))]
    Down,
    /// Facing -Z, 180 degrees
    North,
    /// Facing +Z, 0 degrees
    South,
    /// Facing +X, -90 degrees
    East,
    /// Facing -X, 90 degrees
    West,
}

impl Direction {
    /// An array of all possible [`Direction`]s.
    pub const ALL: [Direction; 6] = [
        Direction::Up,
        Direction::Down,
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

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

    /// Returns an iterator over all possible [`Direction`]s.
    #[must_use]
    #[inline]
    pub fn iter() -> std::array::IntoIter<Self, 6> { Self::ALL.into_iter() }
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

impl From<usize> for Direction {
    #[inline]
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::North,
            3 => Direction::South,
            4 => Direction::East,
            5 => Direction::West,
            _ => panic!("Invalid Direction value: {value}"),
        }
    }
}
impl From<Direction> for usize {
    #[inline]
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::North => 2,
            Direction::South => 3,
            Direction::East => 4,
            Direction::West => 5,
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
