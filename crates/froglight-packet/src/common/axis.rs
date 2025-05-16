//! [`Axis`]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "io")]
use froglight_io::prelude::*;
use glam::{IVec3, Vec3};
use serde::{Deserialize, Serialize};

use super::Direction;

/// The three axes of a 3D coordinate system.
#[expect(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "io", derive(FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Serialize, Deserialize))]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    /// All [`Axis`]s in order.
    pub const ALL: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

    /// Get the [`Vec3`] of the axis.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::Axis;
    /// use glam::Vec3;
    ///
    /// let x = Axis::X.as_vec3();
    /// assert_eq!(x, Vec3::X);
    /// assert_eq!(x, Vec3::new(1.0, 0.0, 0.0));
    ///
    /// let y = Axis::Y.as_vec3();
    /// assert_eq!(y, Vec3::Y);
    /// assert_eq!(y, Vec3::new(0.0, 1.0, 0.0));
    ///
    /// let z = Axis::Z.as_vec3();
    /// assert_eq!(z, Vec3::Z);
    /// assert_eq!(z, Vec3::new(0.0, 0.0, 1.0));
    /// ```
    #[must_use]
    pub const fn as_vec3(self) -> Vec3 {
        match self {
            Axis::X => Vec3::X,
            Axis::Y => Vec3::Y,
            Axis::Z => Vec3::Z,
        }
    }

    /// Get the [`IVec3`] of the axis.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::Axis;
    /// use glam::IVec3;
    ///
    /// let x = Axis::X.as_ivec3();
    /// assert_eq!(x, IVec3::X);
    /// assert_eq!(x, IVec3::new(1, 0, 0));
    ///
    /// let y = Axis::Y.as_ivec3();
    /// assert_eq!(y, IVec3::Y);
    /// assert_eq!(y, IVec3::new(0, 1, 0));
    ///
    /// let z = Axis::Z.as_ivec3();
    /// assert_eq!(z, IVec3::Z);
    /// assert_eq!(z, IVec3::new(0, 0, 1));
    /// ```
    #[must_use]
    pub const fn as_ivec3(self) -> IVec3 {
        match self {
            Axis::X => IVec3::X,
            Axis::Y => IVec3::Y,
            Axis::Z => IVec3::Z,
        }
    }

    /// Select a value based on the [`Axis`].
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::Axis;
    ///
    /// assert_eq!(Axis::X.select(1, 2, 3), 1);
    /// assert_eq!(Axis::Y.select(1, 2, 3), 2);
    /// assert_eq!(Axis::Z.select(1, 2, 3), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn select<T>(self, x: T, y: T, z: T) -> T {
        match self {
            Axis::X => x,
            Axis::Y => y,
            Axis::Z => z,
        }
    }

    /// Select a value from a list based on the [`Axis`].
    ///
    /// # Panics
    /// May panic if the list does not contain at least 3 elements.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::Axis;
    ///
    /// let list = [1, 2, 3];
    ///
    /// let x = Axis::X.select_list(&list);
    /// assert_eq!(x, &1);
    ///
    /// let y = Axis::Y.select_list(&list);
    /// assert_eq!(y, &2);
    ///
    /// let z = Axis::Z.select_list(&list);
    /// assert_eq!(z, &3);
    /// ```
    #[inline]
    #[must_use]
    pub const fn select_list<T>(self, list: &[T]) -> &T {
        match self {
            Axis::X => &list[0],
            Axis::Y => &list[1],
            Axis::Z => &list[2],
        }
    }

    /// Get the nearest [`Axis`] for the given vector.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_packet::common::Axis;
    /// use glam::Vec3;
    ///
    /// assert_eq!(Axis::nearest(Vec3::X), Axis::X);
    /// assert_eq!(Axis::nearest(Vec3::NEG_X), Axis::X);
    ///
    /// assert_eq!(Axis::nearest(Vec3::Y), Axis::Y);
    /// assert_eq!(Axis::nearest(Vec3::NEG_Y), Axis::Y);
    ///
    /// assert_eq!(Axis::nearest(Vec3::Z), Axis::Z);
    /// assert_eq!(Axis::nearest(Vec3::NEG_Z), Axis::Z);
    /// ```
    #[must_use]
    pub fn nearest(vector: Vec3) -> Self {
        match Direction::nearest(vector) {
            Direction::Up | Direction::Down => Axis::Y,
            Direction::North | Direction::South => Axis::Z,
            Direction::West | Direction::East => Axis::X,
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// TODO: Tests
