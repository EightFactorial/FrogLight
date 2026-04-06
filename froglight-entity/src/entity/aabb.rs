use core::ops::{Deref, DerefMut};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_math::bounding::Aabb3d;
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
use froglight_common::aabb::CommonAabb;
use glam::{Vec3, Vec3A};

/// An axis-aligned bounding box (AABB) for an entity.
///
/// Centered around the entity's eye position.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Component))]
pub struct EntityAabb {
    common: CommonAabb,
    eye_height: f32,
}

impl EntityAabb {
    /// Create a new [`EntityAabb`] from an entity's size and eye height.
    #[must_use]
    pub const fn new(xz: f32, y: f32, eye_height: f32) -> Self {
        Self::new_from(CommonAabb::new_centered(Vec3::ZERO, Vec3::new(xz, y, xz)), eye_height)
    }

    /// Create an [`EntityAabb`] from it's [`CommonAabb`] and the entity's eye
    /// height.
    #[inline]
    #[must_use]
    pub const fn new_from(common: CommonAabb, eye_height: f32) -> Self {
        Self { common, eye_height }
    }

    /// Create a new [`EntityAabb`] by translating this one by the given vector.
    #[must_use]
    pub const fn with_translation(mut self, translation: Vec3) -> Self {
        self.common = self.common.with_translation(translation);
        self
    }

    /// Create a new [`EntityAabb`] by scaling this one by the given vector.
    #[must_use]
    pub const fn with_scale(mut self, scale: Vec3) -> Self {
        self.common = self.common.with_scale(scale);
        self
    }

    /// Check if this [`EntityAabb`] intersects with another.
    #[inline]
    #[must_use]
    pub const fn intersects(&self, other: &EntityAabb) -> bool {
        self.common.intersects(&other.common)
    }

    /// Get the inner [`CommonAabb`] of this [`EntityAabb`].
    #[inline]
    #[must_use]
    pub const fn common(&self) -> &CommonAabb { &self.common }

    /// Get the eye height of this [`EntityAabb`].
    #[inline]
    #[must_use]
    pub const fn eye_height(&self) -> f32 { self.eye_height }

    /// Create an [`Aabb3d`] from this [`EntityAabb`].
    #[must_use]
    pub const fn into_aabb(self) -> Aabb3d {
        let (min, max) = self.common.min_max();
        Aabb3d { min: Vec3A::new(min.x, min.y, min.z), max: Vec3A::new(max.x, max.y, max.z) }
    }
}

impl Deref for EntityAabb {
    type Target = CommonAabb;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.common }
}
impl DerefMut for EntityAabb {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.common }
}

impl From<EntityAabb> for CommonAabb {
    #[inline]
    fn from(value: EntityAabb) -> Self { value.common }
}
impl From<EntityAabb> for Aabb3d {
    #[inline]
    fn from(value: EntityAabb) -> Self { value.into_aabb() }
}
