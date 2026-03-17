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
use glam::DVec3;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Using a larger epsilon to match original behavior.
const EPSILON: f64 = 1e-7;

/// An axis-aligned bounding box (AABB).
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(Facet), facet(opaque))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct CommonAabb {
    min: DVec3,
    max: DVec3,
}

impl CommonAabb {
    /// A unit cube AABB from `[0, 0, 0]` to `[1, 1, 1]`.
    pub const ONE: Self = Self { min: DVec3::ZERO, max: DVec3::ONE };
    /// An empty AABB.
    pub const ZERO: Self = Self { min: DVec3::ZERO, max: DVec3::ZERO };

    /// Creates a new [`CommonAabb`] from the given minimum and maximum
    /// coordinates.
    #[must_use]
    pub const fn new(min: DVec3, max: DVec3) -> Self { Self { min, max } }

    /// Creates a new [`CommonAabb`] from the given minimum and maximum
    /// coordinates.
    #[must_use]
    pub const fn new_xyz(
        min_x: f64,
        min_y: f64,
        min_z: f64,
        max_x: f64,
        max_y: f64,
        max_z: f64,
    ) -> Self {
        Self::new(DVec3::new(min_x, min_y, min_z), DVec3::new(max_x, max_y, max_z))
    }

    /// Creates a new [`CommonAabb`] from the given two corner points.
    #[must_use]
    #[cfg(feature = "std")]
    pub const fn new_corners(a: DVec3, b: DVec3) -> Self {
        Self::new_xyz(
            a.x.min(b.x),
            a.y.min(b.y),
            a.z.min(b.z),
            a.x.max(b.x),
            a.y.max(b.y),
            a.z.max(b.z),
        )
    }

    /// Creates a new [`CommonAabb`] from the given two corner points.
    #[must_use]
    #[cfg(all(not(feature = "std"), feature = "libm"))]
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

    /// Creates a new [`CommonAabb`] from the given center and size.
    #[must_use]
    pub const fn new_centered(center: DVec3, size: DVec3) -> Self {
        let half_x = size.x / 2.;
        let half_y = size.y / 2.;
        let half_z = size.z / 2.;
        Self::new(
            DVec3::new(center.x - half_x, center.y - half_y, center.z - half_z),
            DVec3::new(center.x + half_x, center.y + half_y, center.z + half_z),
        )
    }

    /// Compares two [`CommonAabb`]s for equality in a `const` context.
    #[must_use]
    pub const fn const_eq(&self, other: &Self) -> bool {
        (self.min.x - other.min.x).abs() < EPSILON
            && (self.min.y - other.min.y).abs() < EPSILON
            && (self.min.z - other.min.z).abs() < EPSILON
            && (self.max.x - other.max.x).abs() < EPSILON
            && (self.max.y - other.max.y).abs() < EPSILON
            && (self.max.z - other.max.z).abs() < EPSILON
    }

    /// Returns `true` if the given point is contained within the AABB.
    #[must_use]
    pub const fn contains(&self, point: DVec3) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    /// Returns the minimum coordinates of the AABB.
    #[must_use]
    pub const fn min(self) -> DVec3 { self.min }

    /// Returns the maximum coordinates of the AABB.
    #[must_use]
    pub const fn max(self) -> DVec3 { self.max }

    /// Returns the minimum and maximum coordinates of the AABB as a tuple.
    #[must_use]
    pub const fn min_max(self) -> (DVec3, DVec3) { (self.min, self.max) }
}

impl PartialEq for CommonAabb {
    fn eq(&self, other: &Self) -> bool {
        self.min.abs_diff_eq(other.min, EPSILON) && self.max.abs_diff_eq(other.max, EPSILON)
    }
}

impl Add<f64> for CommonAabb {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output { self + DVec3::splat(rhs) }
}
impl Add<DVec3> for CommonAabb {
    type Output = Self;

    fn add(mut self, rhs: DVec3) -> Self::Output {
        self.min += rhs;
        self.max += rhs;
        self
    }
}

impl Sub<f64> for CommonAabb {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output { self - DVec3::splat(rhs) }
}
impl Sub<DVec3> for CommonAabb {
    type Output = Self;

    fn sub(mut self, rhs: DVec3) -> Self::Output {
        self.min -= rhs;
        self.max -= rhs;
        self
    }
}
