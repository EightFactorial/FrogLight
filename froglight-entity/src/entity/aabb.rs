use core::ops::{Deref, DerefMut};

use froglight_common::aabb::CommonAabb;
use glam::DVec3;

/// An axis-aligned bounding box (AABB) for an entity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityAabb {
    common: CommonAabb,
    eye_height: f64,
}

impl EntityAabb {
    /// Create a new [`EntityAabb`] from an entity's size and eye height.
    #[must_use]
    pub const fn new(xz: f64, y: f64, eye_height: f64) -> Self {
        let eye_pos = DVec3::new(0., eye_height, 0.);
        Self::new_from(CommonAabb::new_centered(eye_pos, DVec3::new(xz, y, xz)), eye_height)
    }

    /// Create a new [`EntityAabb`] from a [`CommonAabb`] and an entity's eye
    /// height.
    #[inline]
    #[must_use]
    pub const fn new_from(common: CommonAabb, eye_height: f64) -> Self {
        Self { common, eye_height }
    }

    /// Get the inner [`CommonAabb`] of this [`EntityAabb`].
    #[inline]
    #[must_use]
    pub const fn common(&self) -> &CommonAabb { &self.common }

    /// Get the eye height of this [`EntityAabb`].
    #[inline]
    #[must_use]
    pub const fn eye_height(&self) -> f64 { self.eye_height }
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
