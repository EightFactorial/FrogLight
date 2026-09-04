#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use froglight_common::crates::glam::Vec3;

/// An axis-aligned bounding box (AABB) for an entity.
///
/// Centered around the entity's eye position.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub struct EntityAabb {
    /// The minimum corner of this [`EntityAabb`].
    ///
    /// The `y` of this should always be `0.0`.
    pub min: Vec3,
    /// The maximum corner of this [`EntityAabb`].
    pub max: Vec3,
    /// The eye height of this [`EntityAabb`].
    pub eye_height: f32,
}

impl EntityAabb {
    /// Create a new [`EntityAabb`] from an entity's size and eye height.
    #[must_use]
    pub const fn new(xz: f32, y: f32, eye_height: f32) -> Self {
        let half_xz = xz / 2.;

        Self {
            min: Vec3::new(-half_xz, 0.0, -half_xz),
            max: Vec3::new(half_xz, y, half_xz),
            eye_height,
        }
    }

    /// Get the minimum corner of this [`EntityAabb`].
    #[inline]
    #[must_use]
    pub const fn min(&self) -> Vec3 { self.min }

    /// Get the maximum corner of this [`EntityAabb`].
    #[inline]
    #[must_use]
    pub const fn max(&self) -> Vec3 { self.max }

    /// Get the eye height of this [`EntityAabb`].
    #[inline]
    #[must_use]
    pub const fn eye_height(&self) -> f32 { self.eye_height }
}
