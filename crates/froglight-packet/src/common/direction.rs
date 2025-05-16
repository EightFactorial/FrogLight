//! [`Direction`], [`VerticalDirection`], and [`HorizontalDirection`].

use core::cmp::Ordering;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "io")]
use froglight_io::prelude::*;
use glam::{IVec3, Vec3};
use serde::{Deserialize, Serialize};

/// A direction in 3D space.
#[expect(missing_docs)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "io", derive(FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(
    feature = "bevy",
    reflect(Debug, Default, Clone, PartialEq, Hash, Serialize, Deserialize)
)]
pub enum Direction {
    #[default]
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    /// All [`Direction`]s in order.
    pub const ALL: [Direction; 6] = [
        Direction::Down,
        Direction::Up,
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    /// Get the [`Vec3`] of the direction.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::Direction;
    /// use glam::Vec3;
    ///
    /// let down = Direction::Down.as_vec3();
    /// assert_eq!(down, Vec3::NEG_Y);
    /// assert_eq!(down, Vec3::new(0.0, -1.0, 0.0));
    ///
    /// let up = Direction::Up.as_vec3();
    /// assert_eq!(up, Vec3::Y);
    /// assert_eq!(up, Vec3::new(0.0, 1.0, 0.0));
    ///
    /// let north = Direction::North.as_vec3();
    /// assert_eq!(north, Vec3::NEG_Z);
    /// assert_eq!(north, Vec3::new(0.0, 0.0, -1.0));
    ///
    /// let south = Direction::South.as_vec3();
    /// assert_eq!(south, Vec3::Z);
    /// assert_eq!(south, Vec3::new(0.0, 0.0, 1.0));
    ///
    /// let west = Direction::West.as_vec3();
    /// assert_eq!(west, Vec3::NEG_X);
    /// assert_eq!(west, Vec3::new(-1.0, 0.0, 0.0));
    ///
    /// let east = Direction::East.as_vec3();
    /// assert_eq!(east, Vec3::X);
    /// assert_eq!(east, Vec3::new(1.0, 0.0, 0.0));
    /// ```
    #[must_use]
    pub const fn as_vec3(self) -> Vec3 {
        match self {
            Direction::Down => Vec3::NEG_Y,
            Direction::Up => Vec3::Y,
            Direction::North => Vec3::NEG_Z,
            Direction::South => Vec3::Z,
            Direction::West => Vec3::NEG_X,
            Direction::East => Vec3::X,
        }
    }

    /// Get the [`IVec3`] of the direction.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::Direction;
    /// use glam::IVec3;
    ///
    /// let down = Direction::Down.as_ivec3();
    /// assert_eq!(down, IVec3::NEG_Y);
    /// assert_eq!(down, IVec3::new(0, -1, 0));
    ///
    /// let up = Direction::Up.as_ivec3();
    /// assert_eq!(up, IVec3::Y);
    /// assert_eq!(up, IVec3::new(0, 1, 0));
    ///
    /// let north = Direction::North.as_ivec3();
    /// assert_eq!(north, IVec3::NEG_Z);
    /// assert_eq!(north, IVec3::new(0, 0, -1));
    ///
    /// let south = Direction::South.as_ivec3();
    /// assert_eq!(south, IVec3::Z);
    /// assert_eq!(south, IVec3::new(0, 0, 1));
    ///
    /// let west = Direction::West.as_ivec3();
    /// assert_eq!(west, IVec3::NEG_X);
    /// assert_eq!(west, IVec3::new(-1, 0, 0));
    ///
    /// let east = Direction::East.as_ivec3();
    /// assert_eq!(east, IVec3::X);
    /// assert_eq!(east, IVec3::new(1, 0, 0));
    /// ```
    #[must_use]
    pub const fn as_ivec3(self) -> IVec3 {
        match self {
            Direction::Down => IVec3::NEG_Y,
            Direction::Up => IVec3::Y,
            Direction::North => IVec3::NEG_Z,
            Direction::South => IVec3::Z,
            Direction::West => IVec3::NEG_X,
            Direction::East => IVec3::X,
        }
    }

    /// Get the opposite [`Direction`].
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::Direction;
    ///
    /// assert_eq!(Direction::Down.opposite(), Direction::Up);
    /// assert_eq!(Direction::Up.opposite(), Direction::Down);
    /// assert_eq!(Direction::North.opposite(), Direction::South);
    /// assert_eq!(Direction::South.opposite(), Direction::North);
    /// assert_eq!(Direction::West.opposite(), Direction::East);
    /// assert_eq!(Direction::East.opposite(), Direction::West);
    /// ```
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    /// Get the nearest [`Direction`] for the given vector.
    ///
    /// TODO: Double check this handles `0` and midpoints correctly.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::Direction;
    /// use glam::Vec3;
    ///
    /// assert_eq!(Direction::nearest(Vec3::NEG_Y), Direction::Down);
    /// assert_eq!(Direction::nearest(Vec3::Y), Direction::Up);
    /// assert_eq!(Direction::nearest(Vec3::NEG_Z), Direction::North);
    /// assert_eq!(Direction::nearest(Vec3::Z), Direction::South);
    /// assert_eq!(Direction::nearest(Vec3::NEG_X), Direction::West);
    /// assert_eq!(Direction::nearest(Vec3::X), Direction::East);
    /// ```
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn nearest(vector: Vec3) -> Self {
        Self::ALL
            .into_iter()
            .map(|d| (d, d.as_vec3().dot(vector)))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .expect("This will never be `None`, as there are always 6 directions")
            .0
    }
}

// -------------------------------------------------------------------------------------------------

/// A vertical direction in 3D space.
#[expect(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "io", derive(FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Serialize, Deserialize))]
pub enum VerticalDirection {
    Down,
    Up,
}

impl VerticalDirection {
    /// All [`VerticalDirection`]s in order.
    pub const ALL: [VerticalDirection; 2] = [VerticalDirection::Down, VerticalDirection::Up];

    /// Get the [`Vec3`] of the direction.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::direction::VerticalDirection;
    /// use glam::Vec3;
    ///
    /// let down = VerticalDirection::Down.as_vec3();
    /// assert_eq!(down, Vec3::NEG_Y);
    /// assert_eq!(down, Vec3::new(0.0, -1.0, 0.0));
    ///
    /// let up = VerticalDirection::Up.as_vec3();
    /// assert_eq!(up, Vec3::Y);
    /// assert_eq!(up, Vec3::new(0.0, 1.0, 0.0));
    /// ```
    #[must_use]
    pub const fn as_vec3(self) -> Vec3 {
        match self {
            VerticalDirection::Down => Vec3::NEG_Y,
            VerticalDirection::Up => Vec3::Y,
        }
    }

    /// Get the [`IVec3`] of the direction.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::direction::VerticalDirection;
    /// use glam::IVec3;
    ///
    /// let down = VerticalDirection::Down.as_ivec3();
    /// assert_eq!(down, IVec3::NEG_Y);
    /// assert_eq!(down, IVec3::new(0, -1, 0));
    ///
    /// let up = VerticalDirection::Up.as_ivec3();
    /// assert_eq!(up, IVec3::Y);
    /// assert_eq!(up, IVec3::new(0, 1, 0));
    /// ```
    #[must_use]
    pub const fn as_ivec3(self) -> IVec3 {
        match self {
            VerticalDirection::Down => IVec3::NEG_Y,
            VerticalDirection::Up => IVec3::Y,
        }
    }

    /// Get the opposite [`VerticalDirection`].
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::direction::VerticalDirection;
    ///
    /// assert_eq!(VerticalDirection::Down.opposite(), VerticalDirection::Up);
    /// assert_eq!(VerticalDirection::Up.opposite(), VerticalDirection::Down);
    /// ```
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            VerticalDirection::Down => VerticalDirection::Up,
            VerticalDirection::Up => VerticalDirection::Down,
        }
    }

    /// Get the nearest [`VerticalDirection`] for the given vector.
    ///
    /// TODO: Double check this handles `0` and midpoints correctly.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::direction::VerticalDirection;
    /// use glam::Vec3;
    ///
    /// assert_eq!(VerticalDirection::nearest(Vec3::NEG_Y), VerticalDirection::Down);
    /// assert_eq!(VerticalDirection::nearest(Vec3::Y), VerticalDirection::Up);
    /// ```
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn nearest(vector: Vec3) -> Self {
        Self::ALL
            .into_iter()
            .map(|d| (d, d.as_vec3().dot(vector)))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .expect("This will never be `None`, as there are always 2 directions")
            .0
    }
}

// -------------------------------------------------------------------------------------------------

/// A horizontal direction in 3D space.
///
/// Also known as cardinal directions.
#[expect(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "io", derive(FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Serialize, Deserialize))]
pub enum HorizontalDirection {
    North,
    East,
    South,
    West,
}

impl HorizontalDirection {
    /// All [`HorizontalDirection`]s in order.
    pub const ALL: [HorizontalDirection; 4] = [
        HorizontalDirection::North,
        HorizontalDirection::East,
        HorizontalDirection::South,
        HorizontalDirection::West,
    ];

    /// Get the [`Vec3`] of the direction.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::direction::HorizontalDirection;
    /// use glam::Vec3;
    ///
    /// let north = HorizontalDirection::North.as_vec3();
    /// assert_eq!(north, Vec3::NEG_Z);
    /// assert_eq!(north, Vec3::new(0.0, 0.0, -1.0));
    ///
    /// let south = HorizontalDirection::South.as_vec3();
    /// assert_eq!(south, Vec3::Z);
    /// assert_eq!(south, Vec3::new(0.0, 0.0, 1.0));
    ///
    /// let west = HorizontalDirection::West.as_vec3();
    /// assert_eq!(west, Vec3::NEG_X);
    /// assert_eq!(west, Vec3::new(-1.0, 0.0, 0.0));
    ///
    /// let east = HorizontalDirection::East.as_vec3();
    /// assert_eq!(east, Vec3::X);
    /// assert_eq!(east, Vec3::new(1.0, 0.0, 0.0));
    /// ```
    #[must_use]
    pub const fn as_vec3(self) -> Vec3 {
        match self {
            HorizontalDirection::North => Vec3::NEG_Z,
            HorizontalDirection::South => Vec3::Z,
            HorizontalDirection::West => Vec3::NEG_X,
            HorizontalDirection::East => Vec3::X,
        }
    }

    /// Get the [`IVec3`] of the direction.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::direction::HorizontalDirection;
    /// use glam::IVec3;
    ///
    /// let north = HorizontalDirection::North.as_ivec3();
    /// assert_eq!(north, IVec3::NEG_Z);
    /// assert_eq!(north, IVec3::new(0, 0, -1));
    ///
    /// let south = HorizontalDirection::South.as_ivec3();
    /// assert_eq!(south, IVec3::Z);
    /// assert_eq!(south, IVec3::new(0, 0, 1));
    ///
    /// let west = HorizontalDirection::West.as_ivec3();
    /// assert_eq!(west, IVec3::NEG_X);
    /// assert_eq!(west, IVec3::new(-1, 0, 0));
    ///
    /// let east = HorizontalDirection::East.as_ivec3();
    /// assert_eq!(east, IVec3::X);
    /// assert_eq!(east, IVec3::new(1, 0, 0));
    /// ```
    #[must_use]
    pub const fn as_ivec3(self) -> IVec3 {
        match self {
            HorizontalDirection::North => IVec3::NEG_Z,
            HorizontalDirection::South => IVec3::Z,
            HorizontalDirection::West => IVec3::NEG_X,
            HorizontalDirection::East => IVec3::X,
        }
    }

    /// Get the [`HorizontalDirection`] to the right of this one.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::direction::HorizontalDirection;
    ///
    /// let mut direction = HorizontalDirection::North;
    /// assert_eq!(direction, HorizontalDirection::North);
    ///
    /// direction = direction.right();
    /// assert_eq!(direction, HorizontalDirection::East);
    ///
    /// direction = direction.right();
    /// assert_eq!(direction, HorizontalDirection::South);
    ///
    /// direction = direction.right();
    /// assert_eq!(direction, HorizontalDirection::West);
    ///
    /// direction = direction.right();
    /// assert_eq!(direction, HorizontalDirection::North);
    /// ```
    #[must_use]
    pub const fn right(self) -> Self {
        match self {
            HorizontalDirection::North => HorizontalDirection::East,
            HorizontalDirection::East => HorizontalDirection::South,
            HorizontalDirection::South => HorizontalDirection::West,
            HorizontalDirection::West => HorizontalDirection::North,
        }
    }

    /// Get the [`HorizontalDirection`] to the left of this one.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::direction::HorizontalDirection;
    ///
    /// let mut direction = HorizontalDirection::North;
    /// assert_eq!(direction, HorizontalDirection::North);
    ///
    /// direction = direction.left();
    /// assert_eq!(direction, HorizontalDirection::West);
    ///
    /// direction = direction.left();
    /// assert_eq!(direction, HorizontalDirection::South);
    ///
    /// direction = direction.left();
    /// assert_eq!(direction, HorizontalDirection::East);
    ///
    /// direction = direction.left();
    /// assert_eq!(direction, HorizontalDirection::North);
    /// ```
    #[must_use]
    pub const fn left(self) -> Self {
        match self {
            HorizontalDirection::North => HorizontalDirection::West,
            HorizontalDirection::West => HorizontalDirection::South,
            HorizontalDirection::South => HorizontalDirection::East,
            HorizontalDirection::East => HorizontalDirection::North,
        }
    }

    /// Get the opposite [`HorizontalDirection`].
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::direction::HorizontalDirection;
    ///
    /// assert_eq!(HorizontalDirection::South.opposite(), HorizontalDirection::North);
    /// assert_eq!(HorizontalDirection::North.opposite(), HorizontalDirection::South);
    /// assert_eq!(HorizontalDirection::East.opposite(), HorizontalDirection::West);
    /// assert_eq!(HorizontalDirection::West.opposite(), HorizontalDirection::East);
    /// ```
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            HorizontalDirection::North => HorizontalDirection::South,
            HorizontalDirection::South => HorizontalDirection::North,
            HorizontalDirection::West => HorizontalDirection::East,
            HorizontalDirection::East => HorizontalDirection::West,
        }
    }

    /// Get the nearest [`HorizontalDirection`] for the given vector.
    ///
    /// TODO: Double check this handles `0` and midpoints correctly.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::direction::HorizontalDirection;
    /// use glam::Vec3;
    ///
    /// assert_eq!(HorizontalDirection::nearest(Vec3::NEG_Z), HorizontalDirection::North);
    /// assert_eq!(HorizontalDirection::nearest(Vec3::Z), HorizontalDirection::South);
    /// assert_eq!(HorizontalDirection::nearest(Vec3::NEG_X), HorizontalDirection::West);
    /// assert_eq!(HorizontalDirection::nearest(Vec3::X), HorizontalDirection::East);
    /// ```
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn nearest(vector: Vec3) -> Self {
        Self::ALL
            .into_iter()
            .map(|d| (d, d.as_vec3().dot(vector)))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .expect("This will never be `None`, as there are always 4 directions")
            .0
    }
}

// -------------------------------------------------------------------------------------------------
//
// TODO: Tests
