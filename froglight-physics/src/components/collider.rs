use core::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{
    Reflect, ReflectDeserialize, ReflectSerialize,
    std_traits::{ReflectAdd, ReflectAddAssign, ReflectDefault, ReflectSub, ReflectSubAssign},
};
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
#[cfg_attr(feature = "bevy", reflect(Add, AddAssign, Sub, SubAssign, Serialize, Deserialize))]
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

    /// Get the size of this [`Collider`].
    #[inline]
    #[must_use]
    pub fn size(&self) -> Vec3A { self.max - self.min }

    /// Returns `true` if this [`Collider`] intersects the other.
    #[inline]
    #[must_use]
    pub fn intersects(&self, self_pos: &Position, other: &Self, other_pos: &Position) -> bool {
        let self_pos = self_pos.to_vec3a();
        let other_pos = other_pos.to_vec3a();

        let self_min = self.min + self_pos;
        let self_max = self.max + self_pos;
        let other_min = other.min + other_pos;
        let other_max = other.max + other_pos;

        self_min.cmple(other_max).all() && self_max.cmpge(other_min).all()
    }

    /// Returns `true` if this [`Collider`] contains the other.
    #[inline]
    #[must_use]
    pub fn contains(&self, self_pos: &Position, other: &Self, other_pos: &Position) -> bool {
        let self_pos = self_pos.to_vec3a();
        let other_pos = other_pos.to_vec3a();

        let self_min = self.min + self_pos;
        let self_max = self.max + self_pos;
        let other_min = other.min + other_pos;
        let other_max = other.max + other_pos;

        self_min.cmple(other_min).all() && self_max.cmpge(other_max).all()
    }
}

// -------------------------------------------------------------------------------------------------

impl Add for Collider {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { Self::new(self.min + rhs.min, self.max + rhs.max) }
}
impl AddAssign for Collider {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.min += rhs.min;
        self.max += rhs.max;
    }
}

impl Sub for Collider {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.min - rhs.min, self.max - rhs.max) }
}
impl SubAssign for Collider {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.min -= rhs.min;
        self.max -= rhs.max;
    }
}

// -------------------------------------------------------------------------------------------------

/// The previous tick's [`Collider`].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
#[cfg_attr(feature = "bevy", reflect(Add, AddAssign, Sub, SubAssign, Serialize, Deserialize))]
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

// -------------------------------------------------------------------------------------------------

impl Add for PrevCollider {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output { Self::new_col(self.0 + rhs.0) }
}
impl AddAssign for PrevCollider {
    #[inline]
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

impl Sub for PrevCollider {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output { Self::new_col(self.0 - rhs.0) }
}
impl SubAssign for PrevCollider {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
}
