//! TODO
#![allow(
    clippy::unsafe_derive_deserialize,
    reason = "Allowed, as while important, it does not cause undefined behavior"
)]

use core::ops::{Add, Sub};

#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize};
#[cfg(feature = "facet")]
use facet::Facet;
use glam::{DVec3, Vec3};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Using larger epsilon to match original behavior.
pub const EPSILON_F32: f32 = 1e-7;
/// Using larger epsilon to match original behavior.
pub const EPSILON_F64: f64 = 1e-7;

/// An axis-aligned bounding box.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(Facet), facet(opaque))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct CommonAabb {
    /// The minimum point of the AABB.
    pub min: Vec3,
    /// The maximum point of the AABB.
    pub max: Vec3,
}

/// An axis-aligned bounding box.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(Facet), facet(opaque))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct DCommonAabb {
    /// The minimum point of the AABB.
    pub min: DVec3,
    /// The maximum point of the AABB.
    pub max: DVec3,
}

// -------------------------------------------------------------------------------------------------

macro_rules! impl_aabbs {
    ($ty:ident, $vecty:ident, $valty:ty, $epsilon:expr) => {
        impl $ty {
            /// A unit cube AABB from `[0, 0, 0]` to `[1, 1, 1]`.
            pub const ONE: Self = Self { min: $vecty::ZERO, max: $vecty::ONE };
            /// An empty AABB.
            pub const ZERO: Self = Self { min: $vecty::ZERO, max: $vecty::ZERO };

            #[must_use]
            #[doc = concat!("Creates a new [`", stringify!($ty), "`]from the given minimum and maximum coordinates.")]
            pub const fn new(min: $vecty, max: $vecty) -> Self { Self { min, max } }

            #[must_use]
            #[doc = concat!("Creates a new [`", stringify!($ty), "`]from the given minimum and maximum coordinates.")]
            pub const fn new_xyz(
                min_x: $valty,
                min_y: $valty,
                min_z: $valty,
                max_x: $valty,
                max_y: $valty,
                max_z: $valty,
            ) -> Self {
                Self::new($vecty::new(min_x, min_y, min_z), $vecty::new(max_x, max_y, max_z))
            }

            #[must_use]
            #[cfg(feature = "std")]
            #[doc = concat!("Creates a new [`", stringify!($ty), "`]from the given two corner points.")]
            pub const fn new_corners(a: $vecty, b: $vecty) -> Self {
                Self::new_xyz(
                    a.x.min(b.x),
                    a.y.min(b.y),
                    a.z.min(b.z),
                    a.x.max(b.x),
                    a.y.max(b.y),
                    a.z.max(b.z),
                )
            }

            #[must_use]
            #[doc = concat!("Creates a new [`", stringify!($ty), "`]from the given center and size.")]
            pub const fn new_centered(center: $vecty, size: $vecty) -> Self {
                let half_x = size.x / 2.;
                let half_y = size.y / 2.;
                let half_z = size.z / 2.;
                Self::new(
                    $vecty::new(center.x - half_x, center.y - half_y, center.z - half_z),
                    $vecty::new(center.x + half_x, center.y + half_y, center.z + half_z),
                )
            }

            #[must_use]
            #[doc = concat!("Compares two [`", stringify!($ty), "`]s for equality in a `const` context.")]
            pub const fn const_eq(&self, other: &Self) -> bool {
                (self.min.x - other.min.x).abs() < $epsilon
                    && (self.min.y - other.min.y).abs() < $epsilon
                    && (self.min.z - other.min.z).abs() < $epsilon
                    && (self.max.x - other.max.x).abs() < $epsilon
                    && (self.max.y - other.max.y).abs() < $epsilon
                    && (self.max.z - other.max.z).abs() < $epsilon
            }

            #[must_use]
            #[doc = concat!("Returns `true` if the given point is contained within the [`", stringify!($ty), "`].")]
            pub const fn contains(&self, point: $vecty) -> bool {
                point.x >= self.min.x
                    && point.x <= self.max.x
                    && point.y >= self.min.y
                    && point.y <= self.max.y
                    && point.z >= self.min.z
                    && point.z <= self.max.z
            }

            #[must_use]
            #[doc = concat!("Returns the minimum coordinates of the [`", stringify!($ty), "`].")]
            pub const fn min(self) -> $vecty { self.min }

            #[must_use]
            #[doc = concat!("Returns the maximum coordinates of the [`", stringify!($ty), "`].")]
            pub const fn max(self) -> $vecty { self.max }

            #[must_use]
            #[doc = concat!("Returns the minimum and maximum coordinates of the [`", stringify!($ty), "`] as a tuple.")]
            pub const fn min_max(self) -> ($vecty, $vecty) { (self.min, self.max) }

            /// Get the bottom 4 corners in counter-clockwise order,
            /// starting from the front-left corner.
            #[must_use]
            pub const fn bottom(self) -> [$vecty; 4] {
                let min = self.min;
                let max = self.max;
                [
                    $vecty::new(min.x, min.y, min.z),
                    $vecty::new(max.x, min.y, min.z),
                    $vecty::new(max.x, min.y, max.z),
                    $vecty::new(min.x, min.y, max.z),
                ]
            }

            /// Get the top 4 corners in counter-clockwise order,
            /// starting from the front-left corner.
            #[must_use]
            pub const fn top(self) -> [$vecty; 4] {
                let min = self.min;
                let max = self.max;
                [
                    $vecty::new(max.x, max.y, min.z),
                    $vecty::new(min.x, max.y, min.z),
                    $vecty::new(min.x, max.y, max.z),
                    $vecty::new(max.x, max.y, max.z),
                ]
            }

            /// Get the north 4 corners in counter-clockwise order,
            /// starting from the bottom-left corner.
            #[must_use]
            pub const fn north(self) -> [$vecty; 4] {
                let min = self.min;
                let max = self.max;
                [
                    $vecty::new(max.x, min.y, min.z),
                    $vecty::new(min.x, min.y, min.z),
                    $vecty::new(min.x, max.y, min.z),
                    $vecty::new(max.x, max.y, min.z),
                ]
            }

            /// Get the south 4 corners in counter-clockwise order,
            /// starting from the bottom-left corner.
            #[must_use]
            pub const fn south(self) -> [$vecty; 4] {
                let min = self.min;
                let max = self.max;
                [
                    $vecty::new(min.x, min.y, max.z),
                    $vecty::new(max.x, min.y, max.z),
                    $vecty::new(max.x, max.y, max.z),
                    $vecty::new(min.x, max.y, max.z),
                ]
            }

            /// Get the west 4 corners in counter-clockwise order,
            /// starting from the bottom-front corner.
            #[must_use]
            pub const fn west(self) -> [$vecty; 4] {
                let min = self.min;
                let max = self.max;
                [
                    $vecty::new(min.x, min.y, min.z),
                    $vecty::new(min.x, min.y, max.z),
                    $vecty::new(min.x, max.y, max.z),
                    $vecty::new(min.x, max.y, min.z),
                ]
            }

            /// Get the east 4 corners in counter-clockwise order,
            /// starting from the bottom-front corner.
            #[must_use]
            pub const fn east(self) -> [$vecty; 4] {
                let min = self.min;
                let max = self.max;
                [
                    $vecty::new(max.x, min.y, max.z),
                    $vecty::new(max.x, min.y, min.z),
                    $vecty::new(max.x, max.y, min.z),
                    $vecty::new(max.x, max.y, max.z),
                ]
            }
        }

        impl PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                self.min.abs_diff_eq(other.min, $epsilon) && self.max.abs_diff_eq(other.max, $epsilon)
            }
        }

        impl Add<$valty> for $ty {
            type Output = Self;

            fn add(self, rhs: $valty) -> Self::Output { self + $vecty::splat(rhs) }
        }
        impl Add<$vecty> for $ty {
            type Output = Self;

            fn add(mut self, rhs: $vecty) -> Self::Output {
                self.min += rhs;
                self.max += rhs;
                self
            }
        }

        impl Sub<$valty> for $ty {
            type Output = Self;

            fn sub(self, rhs: $valty) -> Self::Output { self - $vecty::splat(rhs) }
        }
        impl Sub<$vecty> for $ty {
            type Output = Self;

            fn sub(mut self, rhs: $vecty) -> Self::Output {
                self.min -= rhs;
                self.max -= rhs;
                self
            }
        }
    };
}

impl CommonAabb {
    #[must_use]
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    #[doc = concat!("Creates a new [`", stringify!($ty), "`]from the given two corner points.")]
    pub fn new_corners(a: Vec3, b: Vec3) -> Self {
        Self::new_xyz(
            libm::fminf(a.x, b.x),
            libm::fminf(a.y, b.y),
            libm::fminf(a.z, b.z),
            libm::fmaxf(a.x, b.x),
            libm::fmaxf(a.y, b.y),
            libm::fmaxf(a.z, b.z),
        )
    }
}
impl DCommonAabb {
    #[must_use]
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    #[doc = concat!("Creates a new [`", stringify!($ty), "`]from the given two corner points.")]
    pub fn new_corners(a: DVec3, b: DVec3) -> Self {
        Self::new_xyz(
            libm::fmin(a.x, b.x),
            libm::fmin(a.y, b.y),
            libm::fmin(a.z, b.z),
            libm::fmax(a.x, b.x),
            libm::fmax(a.y, b.y),
            libm::fmax(a.z, b.z),
        )
    }
}

impl_aabbs!(CommonAabb, Vec3, f32, EPSILON_F32);
impl_aabbs!(DCommonAabb, DVec3, f64, EPSILON_F64);
