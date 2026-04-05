//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::query::QueryData;
use froglight_entity::prelude::*;

use crate::prelude::*;

/// A bundle of mutable physics component references.
#[cfg_attr(feature = "bevy", derive(QueryData))]
pub struct PhysicsRef<'a> {
    /// The entity's physics controller, if it has one.
    pub controller: Option<&'a PhysicsController>,
    /// The entity's physics state.
    pub state: &'a PhysicsState,
    /// The entity's AABB.
    pub bounding_box: &'a EntityAabb,

    /// The entity's current transform.
    pub transform: &'a Transform,
    /// The entity's previous transform.
    pub prev_transform: &'a PreviousTransform,

    /// The entity's current velocity.
    pub velocity: &'a Velocity,
    /// The entity's previous velocity.
    pub prev_velocity: &'a PreviousVelocity,

    /// The entity's current acceleration.
    pub acceleration: &'a Acceleration,
    /// The entity's previous acceleration.
    pub prev_acceleration: &'a PreviousAcceleration,

    /// Whether the entity is currently on the ground.
    pub on_ground: &'a OnGround,
    /// Whether the entity was previously on the ground.
    pub prev_on_ground: &'a PreviousOnGround,
}

impl<'a> From<PhysicsMut<'a>> for PhysicsRef<'a> {
    #[inline]
    fn from(mutable: PhysicsMut<'a>) -> Self {
        Self {
            controller: mutable.controller.map(|c| &*c),
            state: mutable.state,
            bounding_box: mutable.bounding_box,
            transform: mutable.transform,
            prev_transform: mutable.prev_transform,
            velocity: mutable.velocity,
            prev_velocity: mutable.prev_velocity,
            acceleration: mutable.acceleration,
            prev_acceleration: mutable.prev_acceleration,
            on_ground: mutable.on_ground,
            prev_on_ground: mutable.prev_on_ground,
        }
    }
}
impl<'a> From<&'a PhysicsMut<'_>> for PhysicsRef<'a> {
    #[inline]
    fn from(mutable: &'a PhysicsMut<'_>) -> Self {
        Self {
            controller: mutable.controller.as_deref(),
            state: mutable.state,
            bounding_box: mutable.bounding_box,
            transform: mutable.transform,
            prev_transform: mutable.prev_transform,
            velocity: mutable.velocity,
            prev_velocity: mutable.prev_velocity,
            acceleration: mutable.acceleration,
            prev_acceleration: mutable.prev_acceleration,
            on_ground: mutable.on_ground,
            prev_on_ground: mutable.prev_on_ground,
        }
    }
}

#[cfg(feature = "bevy")]
impl<'a> From<PhysicsRefItem<'a, '_, '_>> for PhysicsRef<'a> {
    #[inline]
    fn from(item: PhysicsRefItem<'a, '_, '_>) -> Self {
        Self {
            controller: item.controller,
            state: item.state,
            bounding_box: item.bounding_box,
            transform: item.transform,
            prev_transform: item.prev_transform,
            velocity: item.velocity,
            prev_velocity: item.prev_velocity,
            acceleration: item.acceleration,
            prev_acceleration: item.prev_acceleration,
            on_ground: item.on_ground,
            prev_on_ground: item.prev_on_ground,
        }
    }
}
#[cfg(feature = "bevy")]
impl<'a> From<&'a PhysicsRefItem<'_, '_, '_>> for PhysicsRef<'a> {
    #[inline]
    fn from(item: &'a PhysicsRefItem<'_, '_, '_>) -> Self {
        Self {
            controller: item.controller,
            state: item.state,
            bounding_box: item.bounding_box,
            transform: item.transform,
            prev_transform: item.prev_transform,
            velocity: item.velocity,
            prev_velocity: item.prev_velocity,
            acceleration: item.acceleration,
            prev_acceleration: item.prev_acceleration,
            on_ground: item.on_ground,
            prev_on_ground: item.prev_on_ground,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A bundle of mutable physics component references.
#[cfg_attr(feature = "bevy", derive(QueryData), query_data(mutable))]
pub struct PhysicsMut<'a> {
    /// The entity's physics controller, if it has one.
    pub controller: Option<&'a mut PhysicsController>,
    /// The entity's physics state.
    pub state: &'a mut PhysicsState,
    /// The entity's AABB.
    pub bounding_box: &'a mut EntityAabb,

    /// The entity's current transform.
    pub transform: &'a mut Transform,
    /// The entity's previous transform.
    pub prev_transform: &'a mut PreviousTransform,

    /// The entity's current velocity.
    pub velocity: &'a mut Velocity,
    /// The entity's previous velocity.
    pub prev_velocity: &'a mut PreviousVelocity,

    /// The entity's current acceleration.
    pub acceleration: &'a mut Acceleration,
    /// The entity's previous acceleration.
    pub prev_acceleration: &'a mut PreviousAcceleration,

    /// Whether the entity is currently on the ground.
    pub on_ground: &'a mut OnGround,
    /// Whether the entity was previously on the ground.
    pub prev_on_ground: &'a mut PreviousOnGround,
}

impl<'a> PhysicsMut<'a> {
    /// Update the previous state to match the current state.
    #[inline]
    pub fn update_previous(&mut self) {
        *self.prev_transform = PreviousTransform(*self.transform);
        *self.prev_velocity = PreviousVelocity(**self.velocity);
        *self.prev_acceleration = PreviousAcceleration(**self.acceleration);
        *self.prev_on_ground = PreviousOnGround(**self.on_ground);
    }

    /// Reborrow this [`PhysicsMut`] as a [`PhysicsRef`].
    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> PhysicsRef<'_> { PhysicsRef::from(self) }

    /// Convert this [`PhysicsMut`] into a [`PhysicsRef`].
    #[inline]
    #[must_use]
    pub fn into_ref(self) -> PhysicsRef<'a> { PhysicsRef::from(self) }
}

#[cfg(feature = "bevy")]
impl<'a> From<PhysicsMutItem<'a, '_, '_>> for PhysicsRef<'a> {
    #[inline]
    fn from(item: PhysicsMutItem<'a, '_, '_>) -> Self {
        Self {
            controller: item.controller.map(|c| &*c.into_inner()),
            state: item.state.into_inner(),
            bounding_box: item.bounding_box.into_inner(),
            transform: item.transform.into_inner(),
            prev_transform: item.prev_transform.into_inner(),
            velocity: item.velocity.into_inner(),
            prev_velocity: item.prev_velocity.into_inner(),
            acceleration: item.acceleration.into_inner(),
            prev_acceleration: item.prev_acceleration.into_inner(),
            on_ground: item.on_ground.into_inner(),
            prev_on_ground: item.prev_on_ground.into_inner(),
        }
    }
}
#[cfg(feature = "bevy")]
impl<'a> From<&'a PhysicsMutItem<'_, '_, '_>> for PhysicsRef<'a> {
    #[inline]
    fn from(item: &'a PhysicsMutItem<'_, '_, '_>) -> Self {
        Self {
            controller: item.controller.as_deref(),
            state: &item.state,
            bounding_box: &item.bounding_box,
            transform: &item.transform,
            prev_transform: &item.prev_transform,
            velocity: &item.velocity,
            prev_velocity: &item.prev_velocity,
            acceleration: &item.acceleration,
            prev_acceleration: &item.prev_acceleration,
            on_ground: &item.on_ground,
            prev_on_ground: &item.prev_on_ground,
        }
    }
}

#[cfg(feature = "bevy")]
impl<'a> From<PhysicsMutItem<'a, '_, '_>> for PhysicsMut<'a> {
    #[inline]
    fn from(item: PhysicsMutItem<'a, '_, '_>) -> Self {
        Self {
            controller: item.controller.map(bevy_ecs::change_detection::Mut::into_inner),
            state: item.state.into_inner(),
            bounding_box: item.bounding_box.into_inner(),
            transform: item.transform.into_inner(),
            prev_transform: item.prev_transform.into_inner(),
            velocity: item.velocity.into_inner(),
            prev_velocity: item.prev_velocity.into_inner(),
            acceleration: item.acceleration.into_inner(),
            prev_acceleration: item.prev_acceleration.into_inner(),
            on_ground: item.on_ground.into_inner(),
            prev_on_ground: item.prev_on_ground.into_inner(),
        }
    }
}
#[cfg(feature = "bevy")]
impl<'a> From<&'a mut PhysicsMutItem<'_, '_, '_>> for PhysicsMut<'a> {
    #[inline]
    fn from(item: &'a mut PhysicsMutItem<'_, '_, '_>) -> Self {
        Self {
            controller: item.controller.as_deref_mut(),
            state: &mut item.state,
            bounding_box: &mut item.bounding_box,
            transform: &mut item.transform,
            prev_transform: &mut item.prev_transform,
            velocity: &mut item.velocity,
            prev_velocity: &mut item.prev_velocity,
            acceleration: &mut item.acceleration,
            prev_acceleration: &mut item.prev_acceleration,
            on_ground: &mut item.on_ground,
            prev_on_ground: &mut item.prev_on_ground,
        }
    }
}

#[cfg(feature = "bevy")]
impl<'a> From<PhysicsMutReadOnlyItem<'a, '_, '_>> for PhysicsRef<'a> {
    #[inline]
    fn from(item: PhysicsMutReadOnlyItem<'a, '_, '_>) -> Self {
        Self {
            controller: item.controller,
            state: item.state,
            bounding_box: item.bounding_box,
            transform: item.transform,
            prev_transform: item.prev_transform,
            velocity: item.velocity,
            prev_velocity: item.prev_velocity,
            acceleration: item.acceleration,
            prev_acceleration: item.prev_acceleration,
            on_ground: item.on_ground,
            prev_on_ground: item.prev_on_ground,
        }
    }
}
#[cfg(feature = "bevy")]
impl<'a> From<&'a PhysicsMutReadOnlyItem<'_, '_, '_>> for PhysicsRef<'a> {
    #[inline]
    fn from(item: &'a PhysicsMutReadOnlyItem<'_, '_, '_>) -> Self {
        Self {
            controller: item.controller,
            state: item.state,
            bounding_box: item.bounding_box,
            transform: item.transform,
            prev_transform: item.prev_transform,
            velocity: item.velocity,
            prev_velocity: item.prev_velocity,
            acceleration: item.acceleration,
            prev_acceleration: item.prev_acceleration,
            on_ground: item.on_ground,
            prev_on_ground: item.prev_on_ground,
        }
    }
}
