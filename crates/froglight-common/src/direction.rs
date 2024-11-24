use std::{fmt::Display, str::FromStr};

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::ReflectDefault;
#[cfg(all(feature = "bevy", feature = "serde"))]
use bevy_reflect::prelude::{ReflectDeserialize, ReflectSerialize};
use glam::{IVec3, Vec3};

/// A direction in 3D space.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Default))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub enum Direction {
    /// Facing -Y
    #[default]
    #[cfg_attr(feature = "serde", serde(alias = "bottom"))]
    Down,
    /// Facing +Y
    #[cfg_attr(feature = "serde", serde(alias = "top"))]
    Up,
    /// Facing -Z, 180 degrees
    North,
    /// Facing +Z, 0 degrees
    South,
    /// Facing -X, 90 degrees
    West,
    /// Facing +X, -90 degrees
    East,
}

impl Direction {
    /// An array of all possible [`Direction`]s.
    pub const ALL: [Direction; 6] = [
        Direction::Down,
        Direction::Up,
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    /// A [`IVec3`] pointing in the [`Direction::Down`] direction.
    pub const DOWN: IVec3 = IVec3::new(0, -1, 0);
    /// A [`IVec3`] pointing in the [`Direction::Up`] direction.
    pub const UP: IVec3 = IVec3::new(0, 1, 0);
    /// A [`IVec3`] pointing in the [`Direction::North`] direction.
    pub const NORTH: IVec3 = IVec3::new(0, 0, -1);
    /// A [`IVec3`] pointing in the [`Direction::South`] direction.
    pub const SOUTH: IVec3 = IVec3::new(0, 0, 1);
    /// A [`IVec3`] pointing in the [`Direction::West`] direction.
    pub const WEST: IVec3 = IVec3::new(-1, 0, 0);
    /// A [`IVec3`] pointing in the [`Direction::East`] direction.
    pub const EAST: IVec3 = IVec3::new(1, 0, 0);

    /// Returns the opposite direction.
    #[inline]
    #[must_use]
    pub const fn opposite(&self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    /// Returns the nearest [`Direction`] to the given [`IVec3`].
    ///
    /// If two directions are equally close, the last one in the
    /// [`Direction::ALL`] array is returned.
    ///
    /// # Example
    /// ```rust
    /// use froglight_common::Direction;
    /// use glam::Vec3;
    ///
    /// let vec = Vec3::new(0., 2., 0.);
    /// assert_eq!(Direction::nearest_from(vec), Direction::Up);
    ///
    /// let vec = Vec3::new(2., 0., 0.);
    /// assert_eq!(Direction::nearest_from(vec), Direction::East);
    ///
    /// let vec = Vec3::splat(0.5);
    /// assert_eq!(Direction::nearest_from(vec), Direction::East);
    ///
    /// for dir in Direction::iter() {
    ///     assert_eq!(Direction::nearest_from(dir.to_axis().as_vec3()), dir);
    ///     assert_eq!(Direction::nearest_from(dir.to_axis().as_vec3() + Vec3::splat(0.1)), dir);
    /// }
    /// ```
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn nearest_from(vec: Vec3) -> Self {
        Self::iter()
            .max_by(|a, b| {
                let a = a.to_axis().as_vec3().dot(vec);
                let b = b.to_axis().as_vec3().dot(vec);
                a.total_cmp(&b)
            })
            .unwrap()
    }

    /// Returns the nearest [`Direction`] to the given [`IVec3`].
    ///
    /// If two directions are equally close, the last one in the
    /// [`Direction::ALL`] array is returned.
    ///
    /// # Example
    /// ```rust
    /// use froglight_common::Direction;
    /// use glam::IVec3;
    ///
    /// let vec = IVec3::new(0, 2, 0);
    /// assert_eq!(Direction::try_from(vec).ok(), None);
    /// assert_eq!(Direction::nearest_from_ivec(vec), Direction::Up);
    ///
    /// let vec = IVec3::new(2, 0, 0);
    /// assert_eq!(Direction::try_from(vec).ok(), None);
    /// assert_eq!(Direction::nearest_from_ivec(vec), Direction::East);
    ///
    /// let vec = IVec3::splat(1);
    /// assert_eq!(Direction::try_from(vec).ok(), None);
    /// assert_eq!(Direction::nearest_from_ivec(vec), Direction::East);
    ///
    /// for dir in Direction::iter() {
    ///     assert_eq!(Direction::try_from(dir.to_axis()).ok(), Some(dir));
    ///     assert_eq!(Direction::nearest_from_ivec(dir.to_axis()), dir);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn nearest_from_ivec(vec: IVec3) -> Self { Self::nearest_from(vec.as_vec3()) }

    /// Returns `true` if this direction is
    /// [`Direction::Down`] or [`Direction::Up`].
    #[inline]
    #[must_use]
    pub const fn is_vertical(&self) -> bool { matches!(self, Direction::Down | Direction::Up) }

    /// Returns `true` if this direction is
    /// [`Direction::North`], [`Direction::South`],
    /// [`Direction::West`], or [`Direction::East`].
    #[inline]
    #[must_use]
    pub const fn is_horizontal(&self) -> bool {
        matches!(self, Direction::North | Direction::South | Direction::West | Direction::East)
    }

    /// Converts a [`Direction`] into an [`IVec3`].
    #[inline]
    #[must_use]
    pub fn to_axis(self) -> IVec3 { IVec3::from(self) }

    /// Attempts to convert an [`IVec3`] into a [`Direction`].
    #[inline]
    #[must_use]
    pub fn try_from_axis(axis: IVec3) -> Option<Direction> { Self::try_from(axis).ok() }

    /// Returns an iterator over all possible [`Direction`]s.
    #[inline]
    #[must_use]
    pub fn iter() -> std::array::IntoIter<Self, 6> { Self::ALL.into_iter() }
}

impl TryFrom<IVec3> for Direction {
    type Error = ();
    #[inline]
    fn try_from(value: IVec3) -> Result<Self, Self::Error> {
        match value {
            Direction::DOWN => Ok(Direction::Down),
            Direction::UP => Ok(Direction::Up),
            Direction::NORTH => Ok(Direction::North),
            Direction::SOUTH => Ok(Direction::South),
            Direction::WEST => Ok(Direction::West),
            Direction::EAST => Ok(Direction::East),
            _ => Err(()),
        }
    }
}
impl From<Direction> for IVec3 {
    #[inline]
    fn from(value: Direction) -> Self {
        match value {
            Direction::Down => Direction::DOWN,
            Direction::Up => Direction::UP,
            Direction::North => Direction::NORTH,
            Direction::South => Direction::SOUTH,
            Direction::West => Direction::WEST,
            Direction::East => Direction::EAST,
        }
    }
}

impl From<usize> for Direction {
    #[inline]
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::Down,
            1 => Direction::Up,
            2 => Direction::North,
            3 => Direction::South,
            4 => Direction::West,
            5 => Direction::East,
            _ => panic!("Invalid Direction value: {value}"),
        }
    }
}
impl From<Direction> for usize {
    #[inline]
    fn from(value: Direction) -> Self {
        match value {
            Direction::Down => 0,
            Direction::Up => 1,
            Direction::North => 2,
            Direction::South => 3,
            Direction::West => 4,
            Direction::East => 5,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Down => write!(f, "Down"),
            Direction::Up => write!(f, "Up"),
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
            Direction::East => write!(f, "East"),
        }
    }
}
impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            "north" => Ok(Direction::North),
            "south" => Ok(Direction::South),
            "west" => Ok(Direction::West),
            "east" => Ok(Direction::East),
            _ => Err(()),
        }
    }
}

impl From<CardinalDirection> for Direction {
    #[inline]
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => Direction::North,
            CardinalDirection::South => Direction::South,
            CardinalDirection::West => Direction::West,
            CardinalDirection::East => Direction::East,
        }
    }
}

/// A cardinal direction in 3D space.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Default))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub enum CardinalDirection {
    /// Facing -Z, 180 degrees
    #[default]
    North,
    /// Facing +Z, 0 degrees
    South,
    /// Facing -X, 90 degrees
    West,
    /// Facing +X, -90 degrees
    East,
}

impl CardinalDirection {
    /// An array of all possible [`CardinalDirection`]s.
    pub const ALL: [CardinalDirection; 4] = [
        CardinalDirection::North,
        CardinalDirection::South,
        CardinalDirection::West,
        CardinalDirection::East,
    ];

    /// A [`IVec3`] pointing in the [`CardinalDirection::North`] direction.
    pub const NORTH: IVec3 = IVec3::new(0, 0, -1);
    /// A [`IVec3`] pointing in the [`CardinalDirection::South`] direction.
    pub const SOUTH: IVec3 = IVec3::new(0, 0, 1);
    /// A [`IVec3`] pointing in the [`CardinalDirection::West`] direction.
    pub const WEST: IVec3 = IVec3::new(-1, 0, 0);
    /// A [`IVec3`] pointing in the [`CardinalDirection::East`] direction.
    pub const EAST: IVec3 = IVec3::new(1, 0, 0);

    /// Returns the opposite direction.
    #[inline]
    #[must_use]
    pub const fn opposite(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::West,
        }
    }

    /// Returns the direction to the right of this one.
    #[inline]
    #[must_use]
    pub const fn right(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::South => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::North,
            CardinalDirection::East => CardinalDirection::South,
        }
    }

    /// Returns the direction to the left of this one.
    #[inline]
    #[must_use]
    pub const fn left(self) -> CardinalDirection {
        match self {
            CardinalDirection::North => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::East,
            CardinalDirection::West => CardinalDirection::South,
            CardinalDirection::East => CardinalDirection::North,
        }
    }

    /// Returns the nearest [`CardinalDirection`] to the given [`Vec3`].
    ///
    /// If two directions are equally close, the last one in the
    /// [`CardinalDirection::ALL`] array is returned.
    ///
    /// # Example
    /// ```rust
    /// use froglight_common::CardinalDirection;
    /// use glam::Vec3;
    ///
    /// let vec = Vec3::new(0., 0., -1.);
    /// assert_eq!(CardinalDirection::nearest_from(vec), CardinalDirection::North);
    ///
    /// let vec = Vec3::new(0., 0., 1.);
    /// assert_eq!(CardinalDirection::nearest_from(vec), CardinalDirection::South);
    ///
    /// let vec = Vec3::splat(1.);
    /// assert_eq!(CardinalDirection::nearest_from(vec), CardinalDirection::East);
    ///
    /// for dir in CardinalDirection::iter() {
    ///     assert_eq!(CardinalDirection::nearest_from(dir.to_axis().as_vec3()), dir);
    ///     assert_eq!(CardinalDirection::nearest_from(dir.to_axis().as_vec3() + Vec3::splat(0.1)), dir);
    /// }
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn nearest_from(vec: Vec3) -> Self {
        Self::iter()
            .max_by(|a, b| {
                let a = a.to_axis().as_vec3().dot(vec);
                let b = b.to_axis().as_vec3().dot(vec);
                a.total_cmp(&b)
            })
            .unwrap()
    }

    /// Returns the nearest [`CardinalDirection`] to the given [`IVec3`].
    ///
    /// If two directions are equally close, the last one in the
    /// [`CardinalDirection::ALL`] array is returned.
    ///
    /// # Example
    /// ```rust
    /// use froglight_common::CardinalDirection;
    /// use glam::IVec3;
    ///
    /// let vec = IVec3::new(0, 0, -2);
    /// assert_eq!(CardinalDirection::try_from(vec).ok(), None);
    /// assert_eq!(CardinalDirection::nearest_from_ivec(vec), CardinalDirection::North);
    ///
    /// let vec = IVec3::new(0, 0, 2);
    /// assert_eq!(CardinalDirection::try_from(vec).ok(), None);
    /// assert_eq!(CardinalDirection::nearest_from_ivec(vec), CardinalDirection::South);
    ///
    /// let vec = IVec3::splat(1);
    /// assert_eq!(CardinalDirection::try_from(vec).ok(), None);
    /// assert_eq!(CardinalDirection::nearest_from_ivec(vec), CardinalDirection::East);
    ///
    /// for dir in CardinalDirection::iter() {
    ///    assert_eq!(CardinalDirection::try_from(dir.to_axis()).ok(), Some(dir));
    ///    assert_eq!(CardinalDirection::nearest_from_ivec(dir.to_axis()), dir);
    /// }
    #[inline]
    #[must_use]
    pub fn nearest_from_ivec(vec: IVec3) -> Self { Self::nearest_from(vec.as_vec3()) }

    /// Converts a [`CardinalDirection`] into an [`IVec3`].
    #[inline]
    #[must_use]
    pub fn to_axis(self) -> IVec3 { IVec3::from(self) }

    /// Attempts to convert an [`IVec3`] into a [`CardinalDirection`].
    #[inline]
    #[must_use]
    pub fn try_from_axis(axis: IVec3) -> Option<CardinalDirection> { Self::try_from(axis).ok() }

    /// Returns an iterator over all possible [`CardinalDirection`]s.
    #[inline]
    #[must_use]
    pub fn iter() -> std::array::IntoIter<Self, 4> { Self::ALL.into_iter() }
}

impl TryFrom<IVec3> for CardinalDirection {
    type Error = ();
    #[inline]
    fn try_from(value: IVec3) -> Result<Self, Self::Error> {
        match value {
            CardinalDirection::NORTH => Ok(CardinalDirection::North),
            CardinalDirection::SOUTH => Ok(CardinalDirection::South),
            CardinalDirection::WEST => Ok(CardinalDirection::West),
            CardinalDirection::EAST => Ok(CardinalDirection::East),
            _ => Err(()),
        }
    }
}
impl From<CardinalDirection> for IVec3 {
    #[inline]
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => CardinalDirection::NORTH,
            CardinalDirection::South => CardinalDirection::SOUTH,
            CardinalDirection::West => CardinalDirection::WEST,
            CardinalDirection::East => CardinalDirection::EAST,
        }
    }
}

impl From<usize> for CardinalDirection {
    #[inline]
    fn from(value: usize) -> Self {
        match value {
            0 => CardinalDirection::North,
            1 => CardinalDirection::South,
            2 => CardinalDirection::West,
            3 => CardinalDirection::East,
            _ => panic!("Invalid CardinalDirection value: {value}"),
        }
    }
}
impl From<CardinalDirection> for usize {
    #[inline]
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => 0,
            CardinalDirection::South => 1,
            CardinalDirection::West => 2,
            CardinalDirection::East => 3,
        }
    }
}

impl Display for CardinalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardinalDirection::North => write!(f, "North"),
            CardinalDirection::South => write!(f, "South"),
            CardinalDirection::West => write!(f, "West"),
            CardinalDirection::East => write!(f, "East"),
        }
    }
}
impl FromStr for CardinalDirection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "north" => Ok(CardinalDirection::North),
            "south" => Ok(CardinalDirection::South),
            "west" => Ok(CardinalDirection::West),
            "east" => Ok(CardinalDirection::East),
            _ => Err(()),
        }
    }
}

impl TryFrom<Direction> for CardinalDirection {
    type Error = ();
    #[inline]
    fn try_from(value: Direction) -> Result<Self, Self::Error> {
        match value {
            Direction::North => Ok(CardinalDirection::North),
            Direction::South => Ok(CardinalDirection::South),
            Direction::West => Ok(CardinalDirection::West),
            Direction::East => Ok(CardinalDirection::East),
            _ => Err(()),
        }
    }
}
