#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
pub use bevy_math::bounding::Aabb3d;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};

/// An entity's collider.
///
/// See [`Aabb3d`] for more information.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "reflect"), reflect(Component))]
pub struct EntityCollider(Aabb3d);

impl EntityCollider {
    /// Create a new [`EntityCollider`].
    #[must_use]
    pub const fn new(collider: Aabb3d) -> Self { Self(collider) }

    /// Create a new [`EntityCollider`] from its center and half-size.
    #[must_use]
    pub fn from_parts(center: impl Into<glam::Vec3A>, half_size: impl Into<glam::Vec3A>) -> Self {
        Self(Aabb3d::new(center, half_size))
    }
}

// -------------------------------------------------------------------------------------------------

/// A 3D axis-aligned bounding box
#[cfg(not(feature = "bevy"))]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub struct Aabb3d {
    /// The minimum point of the box
    pub min: glam::Vec3A,
    /// The maximum point of the box
    pub max: glam::Vec3A,
}

#[cfg(not(feature = "bevy"))]
impl Aabb3d {
    /// Constructs an AABB from its center and half-size.
    #[inline(always)]
    pub fn new(center: impl Into<glam::Vec3A>, half_size: impl Into<glam::Vec3A>) -> Self {
        let (center, half_size) = (center.into(), half_size.into());
        debug_assert!(half_size.x >= 0.0 && half_size.y >= 0.0 && half_size.z >= 0.0);

        Self { min: center - half_size, max: center + half_size }
    }

    /// Returns the center of the bounding volume.
    #[inline(always)]
    pub fn center(&self) -> glam::Vec3A { (self.min + self.max) / 2. }

    /// Returns the half size of the bounding volume.
    #[inline(always)]
    pub fn half_size(&self) -> glam::Vec3A { (self.max - self.min) / 2. }

    /// Computes the visible surface area of the bounding volume.
    /// This method can be useful to make decisions about merging bounding
    /// volumes, using a Surface Area Heuristic.
    ///
    /// For 2D shapes this would simply be the area of the shape.
    /// For 3D shapes this would usually be half the area of the shape.
    #[inline(always)]
    pub fn visible_area(&self) -> f32 {
        let b = self.max - self.min;
        b.x * (b.y + b.z) + b.y * b.z
    }

    /// Checks if this bounding volume contains another one.
    #[inline(always)]
    pub fn contains(&self, other: &Self) -> bool {
        other.min.cmpge(self.min).all() && other.max.cmple(self.max).all()
    }

    /// Check if a volume intersects with this bounding volume.
    #[inline(always)]
    pub fn intersects(&self, other: &Self) -> bool {
        self.min.cmple(other.max).all() && self.max.cmpge(other.min).all()
    }
}

// -------------------------------------------------------------------------------------------------

/// The height of an entity's eyes from the bottom of the [`EntityCollider`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "reflect"), reflect(Component))]
pub struct EntityEyeHeight(f32);

impl EntityEyeHeight {
    /// Create a new [`EntityEyeHeight`].
    #[must_use]
    pub const fn new(eye_height: f32) -> Self {
        debug_assert!(eye_height >= 0.0);
        Self(eye_height)
    }
}

// -------------------------------------------------------------------------------------------------

/// The force of gravity applied to an entity.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(all(feature = "bevy", feature = "reflect"), reflect(Component))]
pub struct EntityGravity(f32);

impl EntityGravity {
    /// Create a new [`EntityGravity`].
    #[inline]
    #[must_use]
    pub const fn new(gravity: f32) -> Self { Self(gravity) }
}
