#[cfg(feature = "bevy")]
use bevy_math::bounding::Aabb3d;
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use glam::Vec3;
#[cfg(feature = "bevy")]
use glam::Vec3A;

/// An axis-aligned bounding box (AABB) for an entity.
///
/// Centered around the entity's eye position.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
pub struct EntityAabb {
    /// The minimum corner of this [`EntityAabb`].
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
        let half_y = y / 2.;

        Self {
            min: Vec3::new(-half_xz, -half_y, -half_xz),
            max: Vec3::new(half_xz, half_y, half_xz),
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

    /// Create an [`Aabb3d`] from this [`EntityAabb`].
    #[must_use]
    #[cfg(feature = "bevy")]
    pub const fn into_aabb(self) -> Aabb3d {
        let Self { min, max, .. } = self;
        Aabb3d { min: Vec3A::new(min.x, min.y, min.z), max: Vec3A::new(max.x, max.y, max.z) }
    }
}

#[cfg(feature = "bevy")]
impl From<EntityAabb> for Aabb3d {
    #[inline]
    fn from(value: EntityAabb) -> Self { value.into_aabb() }
}
