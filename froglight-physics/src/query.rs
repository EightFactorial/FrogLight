//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::query::QueryData;
use froglight_entity::{entity::EntityAttributeSet, prelude::*};

use crate::prelude::*;

/// A bundle of mutable physics component references.
#[cfg_attr(feature = "bevy", derive(QueryData))]
pub struct PhysicsRef<'a> {
    /// The entity's physics controller, if it has one.
    pub controller: Option<&'a PhysicsController>,
    /// The entity's physics state.
    pub state: &'a PhysicsState,

    /// The entity's attributes.
    pub attributes: &'a EntityAttributeSet,
    /// The entity's data bundle.
    pub bundle: &'a EntityBundle,
    /// The entity's AABB.
    pub bounding_box: &'a EntityAabb,

    /// The entity's current world collision state.
    pub world_collision: &'a WorldCollision,
    /// The entity's previous world collision state.
    pub prev_world_collision: &'a PreviousWorldCollision,

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

    /// The entity's current fall distance.
    pub fall_distance: &'a IsFalling,
    /// The entity's previous fall distance.
    pub prev_fall_distance: &'a PreviousIsFalling,

    /// Whether the entity is currently in a fluid.
    pub in_fluid: &'a InFluid,
    /// Whether the entity was previously in a fluid.
    pub prev_in_fluid: &'a PreviousInFluid,
}

impl PhysicsRef<'_> {
    /// Reborrow this [`PhysicsRef`] as a new [`PhysicsRef`] with a shorter
    /// lifetime.
    #[must_use]
    pub const fn reborrow(&self) -> PhysicsRef<'_> {
        PhysicsRef {
            controller: match &self.controller {
                Some(controller) => Some(controller),
                None => None,
            },
            state: self.state,
            attributes: self.attributes,
            bundle: self.bundle,
            world_collision: self.world_collision,
            prev_world_collision: self.prev_world_collision,
            bounding_box: self.bounding_box,
            transform: self.transform,
            prev_transform: self.prev_transform,
            velocity: self.velocity,
            prev_velocity: self.prev_velocity,
            acceleration: self.acceleration,
            prev_acceleration: self.prev_acceleration,
            on_ground: self.on_ground,
            prev_on_ground: self.prev_on_ground,
            fall_distance: self.fall_distance,
            prev_fall_distance: self.prev_fall_distance,
            in_fluid: self.in_fluid,
            prev_in_fluid: self.prev_in_fluid,
        }
    }
}

impl<'a> From<PhysicsMut<'a>> for PhysicsRef<'a> {
    #[inline]
    fn from(mutable: PhysicsMut<'a>) -> Self {
        Self {
            controller: mutable.controller.map(|c| &*c),
            state: mutable.state,
            attributes: mutable.attributes,
            bundle: mutable.bundle,
            world_collision: mutable.world_collision,
            prev_world_collision: mutable.prev_world_collision,
            bounding_box: mutable.bounding_box,
            transform: mutable.transform,
            prev_transform: mutable.prev_transform,
            velocity: mutable.velocity,
            prev_velocity: mutable.prev_velocity,
            acceleration: mutable.acceleration,
            prev_acceleration: mutable.prev_acceleration,
            on_ground: mutable.on_ground,
            prev_on_ground: mutable.prev_on_ground,
            fall_distance: mutable.fall_distance,
            prev_fall_distance: mutable.prev_fall_distance,
            in_fluid: mutable.in_fluid,
            prev_in_fluid: mutable.prev_in_fluid,
        }
    }
}
impl<'a> From<&'a PhysicsMut<'_>> for PhysicsRef<'a> {
    #[inline]
    fn from(mutable: &'a PhysicsMut<'_>) -> Self {
        Self {
            controller: mutable.controller.as_deref(),
            state: mutable.state,
            attributes: mutable.attributes,
            bundle: mutable.bundle,
            world_collision: mutable.world_collision,
            prev_world_collision: mutable.prev_world_collision,
            bounding_box: mutable.bounding_box,
            transform: mutable.transform,
            prev_transform: mutable.prev_transform,
            velocity: mutable.velocity,
            prev_velocity: mutable.prev_velocity,
            acceleration: mutable.acceleration,
            prev_acceleration: mutable.prev_acceleration,
            on_ground: mutable.on_ground,
            prev_on_ground: mutable.prev_on_ground,
            fall_distance: mutable.fall_distance,
            prev_fall_distance: mutable.prev_fall_distance,
            in_fluid: mutable.in_fluid,
            prev_in_fluid: mutable.prev_in_fluid,
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
            attributes: item.attributes,
            bundle: item.bundle,
            world_collision: item.world_collision,
            prev_world_collision: item.prev_world_collision,
            bounding_box: item.bounding_box,
            transform: item.transform,
            prev_transform: item.prev_transform,
            velocity: item.velocity,
            prev_velocity: item.prev_velocity,
            acceleration: item.acceleration,
            prev_acceleration: item.prev_acceleration,
            on_ground: item.on_ground,
            prev_on_ground: item.prev_on_ground,
            fall_distance: item.fall_distance,
            prev_fall_distance: item.prev_fall_distance,
            in_fluid: item.in_fluid,
            prev_in_fluid: item.prev_in_fluid,
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
            attributes: item.attributes,
            bundle: item.bundle,
            world_collision: item.world_collision,
            prev_world_collision: item.prev_world_collision,
            bounding_box: item.bounding_box,
            transform: item.transform,
            prev_transform: item.prev_transform,
            velocity: item.velocity,
            prev_velocity: item.prev_velocity,
            acceleration: item.acceleration,
            prev_acceleration: item.prev_acceleration,
            on_ground: item.on_ground,
            prev_on_ground: item.prev_on_ground,
            fall_distance: item.fall_distance,
            prev_fall_distance: item.prev_fall_distance,
            in_fluid: item.in_fluid,
            prev_in_fluid: item.prev_in_fluid,
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

    /// The entity's attributes.
    pub attributes: &'a mut EntityAttributeSet,
    /// The entity's data bundle.
    pub bundle: &'a mut EntityBundle,
    /// The entity's AABB.
    pub bounding_box: &'a mut EntityAabb,

    /// The entity's current world collision state.
    pub world_collision: &'a mut WorldCollision,
    /// The entity's previous world collision state.
    pub prev_world_collision: &'a mut PreviousWorldCollision,

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

    /// The entity's current fall distance.
    pub fall_distance: &'a mut IsFalling,
    /// The entity's previous fall distance.
    pub prev_fall_distance: &'a mut PreviousIsFalling,

    /// Whether the entity is currently in a fluid.
    pub in_fluid: &'a mut InFluid,
    /// Whether the entity was previously in a fluid.
    pub prev_in_fluid: &'a mut PreviousInFluid,
}

impl<'a> PhysicsMut<'a> {
    /// Get a [`PhysicsMut`] for the given entity from the world.
    ///
    /// Returns `None` if the entity doesn't have the required components.
    #[cfg(feature = "bevy")]
    pub fn from_world(
        entity: bevy_ecs::prelude::Entity,
        world: &mut bevy_ecs::prelude::World,
    ) -> Option<PhysicsMut<'_>> {
        world.query::<PhysicsMut<'_>>().get_mut(world, entity).ok().map(Into::into)
    }

    /// Update the previous state to match the current state.
    #[inline]
    pub fn update_previous(&mut self) {
        *self.prev_world_collision = PreviousWorldCollision(*self.world_collision);
        *self.prev_transform = PreviousTransform(*self.transform);
        *self.prev_velocity = PreviousVelocity(**self.velocity);
        *self.prev_acceleration = PreviousAcceleration(**self.acceleration);
        *self.prev_on_ground = PreviousOnGround(**self.on_ground);
        *self.prev_fall_distance = PreviousIsFalling::from(*self.fall_distance);
        *self.prev_in_fluid = PreviousInFluid::from(*self.in_fluid);
    }

    /// Reborrow this [`PhysicsMut`] as a new [`PhysicsMut`] with a shorter
    /// lifetime.
    #[must_use]
    pub const fn reborrow(&mut self) -> PhysicsMut<'_> {
        PhysicsMut {
            controller: match &mut self.controller {
                Some(controller) => Some(controller),
                None => None,
            },
            state: &mut *self.state,
            attributes: &mut *self.attributes,
            bundle: &mut *self.bundle,
            world_collision: &mut *self.world_collision,
            prev_world_collision: &mut *self.prev_world_collision,
            bounding_box: &mut *self.bounding_box,
            transform: &mut *self.transform,
            prev_transform: &mut *self.prev_transform,
            velocity: &mut *self.velocity,
            prev_velocity: &mut *self.prev_velocity,
            acceleration: &mut *self.acceleration,
            prev_acceleration: &mut *self.prev_acceleration,
            on_ground: &mut *self.on_ground,
            prev_on_ground: &mut *self.prev_on_ground,
            fall_distance: &mut *self.fall_distance,
            prev_fall_distance: &mut *self.prev_fall_distance,
            in_fluid: &mut *self.in_fluid,
            prev_in_fluid: &mut *self.prev_in_fluid,
        }
    }

    /// Reborrow this [`PhysicsMut`] as a [`PhysicsRef`].
    #[must_use]
    pub const fn as_ref(&self) -> PhysicsRef<'_> {
        PhysicsRef {
            controller: match &self.controller {
                Some(controller) => Some(controller),
                None => None,
            },
            state: &*self.state,
            attributes: &*self.attributes,
            bundle: &*self.bundle,
            world_collision: &*self.world_collision,
            prev_world_collision: &*self.prev_world_collision,
            bounding_box: &*self.bounding_box,
            transform: &*self.transform,
            prev_transform: &*self.prev_transform,
            velocity: &*self.velocity,
            prev_velocity: &*self.prev_velocity,
            acceleration: &*self.acceleration,
            prev_acceleration: &*self.prev_acceleration,
            on_ground: &*self.on_ground,
            prev_on_ground: &*self.prev_on_ground,
            fall_distance: &*self.fall_distance,
            prev_fall_distance: &*self.prev_fall_distance,
            in_fluid: &*self.in_fluid,
            prev_in_fluid: &*self.prev_in_fluid,
        }
    }

    /// Convert this [`PhysicsMut`] into a [`PhysicsRef`].
    #[must_use]
    pub const fn into_ref(self) -> PhysicsRef<'a> {
        PhysicsRef {
            controller: match self.controller {
                Some(controller) => Some(controller),
                None => None,
            },
            state: self.state,
            attributes: self.attributes,
            bundle: self.bundle,
            world_collision: self.world_collision,
            prev_world_collision: self.prev_world_collision,
            bounding_box: self.bounding_box,
            transform: self.transform,
            prev_transform: self.prev_transform,
            velocity: self.velocity,
            prev_velocity: self.prev_velocity,
            acceleration: self.acceleration,
            prev_acceleration: self.prev_acceleration,
            on_ground: self.on_ground,
            prev_on_ground: self.prev_on_ground,
            fall_distance: self.fall_distance,
            prev_fall_distance: self.prev_fall_distance,
            in_fluid: self.in_fluid,
            prev_in_fluid: self.prev_in_fluid,
        }
    }
}

#[cfg(feature = "bevy")]
impl<'a> From<PhysicsMutItem<'a, '_, '_>> for PhysicsRef<'a> {
    #[inline]
    fn from(item: PhysicsMutItem<'a, '_, '_>) -> Self {
        Self {
            controller: item.controller.map(|c| &*c.into_inner()),
            state: item.state.into_inner(),
            attributes: item.attributes.into_inner(),
            bundle: item.bundle.into_inner(),
            world_collision: item.world_collision.into_inner(),
            prev_world_collision: item.prev_world_collision.into_inner(),
            bounding_box: item.bounding_box.into_inner(),
            transform: item.transform.into_inner(),
            prev_transform: item.prev_transform.into_inner(),
            velocity: item.velocity.into_inner(),
            prev_velocity: item.prev_velocity.into_inner(),
            acceleration: item.acceleration.into_inner(),
            prev_acceleration: item.prev_acceleration.into_inner(),
            on_ground: item.on_ground.into_inner(),
            prev_on_ground: item.prev_on_ground.into_inner(),
            fall_distance: item.fall_distance.into_inner(),
            prev_fall_distance: item.prev_fall_distance.into_inner(),
            in_fluid: item.in_fluid.into_inner(),
            prev_in_fluid: item.prev_in_fluid.into_inner(),
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
            attributes: &item.attributes,
            bundle: &item.bundle,
            world_collision: &item.world_collision,
            prev_world_collision: &item.prev_world_collision,
            bounding_box: &item.bounding_box,
            transform: &item.transform,
            prev_transform: &item.prev_transform,
            velocity: &item.velocity,
            prev_velocity: &item.prev_velocity,
            acceleration: &item.acceleration,
            prev_acceleration: &item.prev_acceleration,
            on_ground: &item.on_ground,
            prev_on_ground: &item.prev_on_ground,
            fall_distance: &item.fall_distance,
            prev_fall_distance: &item.prev_fall_distance,
            in_fluid: &item.in_fluid,
            prev_in_fluid: &item.prev_in_fluid,
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
            attributes: item.attributes.into_inner(),
            bundle: item.bundle.into_inner(),
            world_collision: item.world_collision.into_inner(),
            prev_world_collision: item.prev_world_collision.into_inner(),
            bounding_box: item.bounding_box.into_inner(),
            transform: item.transform.into_inner(),
            prev_transform: item.prev_transform.into_inner(),
            velocity: item.velocity.into_inner(),
            prev_velocity: item.prev_velocity.into_inner(),
            acceleration: item.acceleration.into_inner(),
            prev_acceleration: item.prev_acceleration.into_inner(),
            on_ground: item.on_ground.into_inner(),
            prev_on_ground: item.prev_on_ground.into_inner(),
            fall_distance: item.fall_distance.into_inner(),
            prev_fall_distance: item.prev_fall_distance.into_inner(),
            in_fluid: item.in_fluid.into_inner(),
            prev_in_fluid: item.prev_in_fluid.into_inner(),
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
            attributes: &mut item.attributes,
            bundle: &mut item.bundle,
            world_collision: &mut item.world_collision,
            prev_world_collision: &mut item.prev_world_collision,
            bounding_box: &mut item.bounding_box,
            transform: &mut item.transform,
            prev_transform: &mut item.prev_transform,
            velocity: &mut item.velocity,
            prev_velocity: &mut item.prev_velocity,
            acceleration: &mut item.acceleration,
            prev_acceleration: &mut item.prev_acceleration,
            on_ground: &mut item.on_ground,
            prev_on_ground: &mut item.prev_on_ground,
            fall_distance: &mut item.fall_distance,
            prev_fall_distance: &mut item.prev_fall_distance,
            in_fluid: &mut item.in_fluid,
            prev_in_fluid: &mut item.prev_in_fluid,
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
            attributes: item.attributes,
            bundle: item.bundle,
            world_collision: item.world_collision,
            prev_world_collision: item.prev_world_collision,
            bounding_box: item.bounding_box,
            transform: item.transform,
            prev_transform: item.prev_transform,
            velocity: item.velocity,
            prev_velocity: item.prev_velocity,
            acceleration: item.acceleration,
            prev_acceleration: item.prev_acceleration,
            on_ground: item.on_ground,
            prev_on_ground: item.prev_on_ground,
            fall_distance: item.fall_distance,
            prev_fall_distance: item.prev_fall_distance,
            in_fluid: item.in_fluid,
            prev_in_fluid: item.prev_in_fluid,
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
            attributes: item.attributes,
            bundle: item.bundle,
            world_collision: item.world_collision,
            prev_world_collision: item.prev_world_collision,
            bounding_box: item.bounding_box,
            transform: item.transform,
            prev_transform: item.prev_transform,
            velocity: item.velocity,
            prev_velocity: item.prev_velocity,
            acceleration: item.acceleration,
            prev_acceleration: item.prev_acceleration,
            on_ground: item.on_ground,
            prev_on_ground: item.prev_on_ground,
            fall_distance: item.fall_distance,
            prev_fall_distance: item.prev_fall_distance,
            in_fluid: item.in_fluid,
            prev_in_fluid: item.prev_in_fluid,
        }
    }
}
