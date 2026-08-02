use core::ops::{Deref, DerefMut};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, ReflectDeserialize, ReflectSerialize, std_traits::ReflectDefault};
use froglight_entity::prelude::EntityAabb;
use glam::Vec3A;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy")]
use crate::prelude::{CollidingWith, Position, Rotation};

/// An entity collider.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", require(Position, Rotation, CollidingWith, PrevCollider))]
pub struct Collider {
    /// The minimum corner of this [`Collider`].
    pub min: Vec3A,
    /// The maximum corner of this [`Collider`].
    pub max: Vec3A,
}

impl Collider {
    /// Create a new [`Collider`] from two [`Vec3A`]s.
    #[inline]
    #[must_use]
    pub const fn new(min: Vec3A, max: Vec3A) -> Self { Self { min, max } }

    /// Create a new [`Collider`] from a center and half-size.
    #[inline]
    #[must_use]
    pub fn new_centered(center: Vec3A, half_size: Vec3A) -> Self {
        Self::new(center - half_size, center + half_size)
    }

    /// Create a new [`Collider`] from an [`EntityAabb`].
    #[inline]
    #[must_use]
    pub const fn new_entity(entity: EntityAabb) -> Self {
        let [min_x, min_y, min_z] = entity.min.to_array();
        let [max_x, max_y, max_z] = entity.max.to_array();
        Self::new(Vec3A::new(min_x, min_y, min_z), Vec3A::new(max_x, max_y, max_z))
    }

    /// Get the size of this [`Collider`].
    #[inline]
    #[must_use]
    pub const fn size(&self) -> Vec3A {
        let [max_x, max_y, max_z] = self.max.to_array();
        let [min_x, min_y, min_z] = self.min.to_array();
        Vec3A::new(max_x - min_x, max_y - min_y, max_z - min_z)
    }

    /// Get the width of this [`Collider`].
    #[inline]
    #[must_use]
    pub const fn width(&self) -> f32 {
        let [max_x, _, max_z] = self.max.to_array();
        let [min_x, _, min_z] = self.min.to_array();
        f32::max(max_x - min_x, max_z - min_z)
    }

    /// Get the height of this [`Collider`].
    #[inline]
    #[must_use]
    pub const fn height(&self) -> f32 {
        let [_, max_y, _] = self.max.to_array();
        let [_, min_y, _] = self.min.to_array();
        max_y - min_y
    }

    /// Get the "actual" center of this [`Collider`],
    /// or the center of the entity's bounding box.
    ///
    /// Also see [`Collider::center_canonical`].
    #[inline]
    #[must_use]
    pub fn center_actual(&self) -> Vec3A { self.min.midpoint(self.max) }

    /// Get the "canonical" center of this [`Collider`],
    /// or the center of the entity's bounding box at it's feet.
    ///
    /// This is used when the server and client communicate an entity's
    /// position.
    ///
    /// Also see [`Collider::center_actual`].
    #[inline]
    #[must_use]
    pub fn center_canonical(&self) -> Vec3A { self.center_actual().with_y(self.min.y) }

    /// Set the position of this [`Collider`].
    #[inline]
    pub fn set_position(&mut self, position: Vec3A) {
        *self = Self::new_centered(position, self.size() / 2.0);
    }

    /// Translate this [`Collider`] by the given [`Vec3A`].
    #[inline]
    pub fn translate(&mut self, translation: Vec3A) {
        self.min += translation;
        self.max += translation;
    }

    /// Returns `true` if this [`Collider`] intersects the other.
    #[inline]
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        self.min.cmple(other.max).all() && self.max.cmpge(other.min).all()
    }

    /// Returns `true` if this [`Collider`] contains the other.
    #[inline]
    #[must_use]
    pub fn contains(&self, other: &Self) -> bool {
        self.min.cmple(other.min).all() && self.max.cmpge(other.max).all()
    }
}

// -------------------------------------------------------------------------------------------------

/// The previous tick's [`Collider`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Serialize, Deserialize))]
pub struct PrevCollider(Collider);

impl PrevCollider {
    /// Create a new [`PrevCollider`] from two [`Vec3A`]s.
    #[inline]
    #[must_use]
    pub const fn new(min: Vec3A, max: Vec3A) -> Self { Self(Collider::new(min, max)) }

    /// Create a new [`PrevCollider`] from a center and half-size.
    #[inline]
    #[must_use]
    pub fn new_centered(center: Vec3A, half_size: Vec3A) -> Self {
        Self(Collider::new_centered(center, half_size))
    }

    /// Create a new [`PrevCollider`] from an [`EntityAabb`].
    #[inline]
    #[must_use]
    pub fn new_entity(entity: EntityAabb) -> Self { Self(Collider::new_entity(entity)) }

    /// Create a new [`PrevCollider`] from a [`Collider`].
    #[inline]
    #[must_use]
    pub const fn new_col(collider: Collider) -> Self { Self(collider) }

    /// Get the minimum corner of this [`PrevCollider`].
    #[inline]
    #[must_use]
    pub const fn min(&self) -> Vec3A { self.0.min }

    /// Get the maximum corner of this [`PrevCollider`].
    #[inline]
    #[must_use]
    pub const fn max(&self) -> Vec3A { self.0.max }
}

impl From<Collider> for PrevCollider {
    #[inline]
    fn from(collider: Collider) -> Self { Self(collider) }
}

impl AsRef<Collider> for PrevCollider {
    #[inline]
    fn as_ref(&self) -> &Collider { &self.0 }
}
impl AsMut<Collider> for PrevCollider {
    #[inline]
    fn as_mut(&mut self) -> &mut Collider { &mut self.0 }
}

impl Deref for PrevCollider {
    type Target = Collider;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for PrevCollider {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
