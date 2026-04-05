//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::query::QueryData;
use froglight_entity::prelude::*;

use crate::prelude::*;

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

impl PhysicsMut<'_> {
    /// Update the previous state to match the current state.
    #[inline]
    pub fn update_previous(&mut self) {
        *self.prev_transform = PreviousTransform(*self.transform);
        *self.prev_velocity = PreviousVelocity(**self.velocity);
        *self.prev_acceleration = PreviousAcceleration(**self.acceleration);
        *self.prev_on_ground = PreviousOnGround(**self.on_ground);
    }
}
