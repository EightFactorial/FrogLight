//! TODO

use bevy_app::{App, Plugin};
#[cfg(feature = "tracing")]
use bevy_ecs::entity::EntityNotSpawnedError;
use bevy_ecs::{prelude::*, world::DeferredWorld};
use bevy_tasks::ComputeTaskPool;
use froglight_entity::{bevy::EntityBundleEvent, prelude::EntityBundle};

use crate::prelude::*;

mod cache;
pub use cache::{CollidingWith, EntityCollisions};

/// A [`Plugin`] that adds physics components and systems.
///
/// # Warning
///
/// This [`Plugin`] includes several [`System`]s that are not scheduled by
/// default!
///
/// This is to allow maximum flexibility when
/// integrating with custom simulation and tick-rates.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Acceleration>().register_type::<PrevAcceleration>();
        app.register_into_type_conversion::<Acceleration, PrevAcceleration>();

        app.register_type::<Position>().register_type::<PrevPosition>();
        app.register_into_type_conversion::<Position, PrevPosition>();

        app.register_type::<Rotation>().register_type::<PrevRotation>();
        app.register_into_type_conversion::<Rotation, PrevRotation>();

        app.register_type::<Velocity>().register_type::<PrevVelocity>();
        app.register_into_type_conversion::<Velocity, PrevVelocity>();

        app.register_type::<Collider>().register_type::<PrevCollider>();
        app.register_into_type_conversion::<Collider, PrevCollider>();

        app.register_type::<EntityCollisions>().init_resource::<EntityCollisions>();
        app.register_type::<CollidingWith>();

        app.add_observer(PhysicsPlugin::on_entity_bundle);
    }
}

impl PhysicsPlugin {
    /// An [`Observer`] that inserts physics components when an entity with an
    /// [`EntityBundle`] is spawned.
    pub fn on_entity_bundle(trigger: On<EntityBundleEvent>, mut world: DeferredWorld) {
        let (entities, mut commands) = world.entities_and_commands();
        let entity_id = trigger.entity();

        match entities.get(entity_id) {
            Ok(entity) => {
                if let Some(bundle) = entity.get::<EntityBundle>() {
                    let mut commands = commands.entity(entity_id);

                    // Insert `Collider`, `Position`, and `Rotation`
                    // (preserves `Position` and `Rotation` if they already exist)
                    let collider = Collider::new_entity(*bundle.metadata().aabb());
                    commands.insert(collider);

                    // Insert `Velocity` and `Acceleration`
                    // (overwrites `Velocity` and `Acceleration` to zero)
                    commands.insert((Velocity::default(), Acceleration::default()));
                } else {
                    #[cfg(feature = "tracing")]
                    tracing::error!(target: "froglight_physics", "Failed to add Collider to Entity {entity_id}, missing EntityBundle component?");
                }
            }
            #[cfg(feature = "tracing")]
            Err(EntityNotSpawnedError::Invalid(..)) => {
                tracing::error!(target: "froglight_physics", "Failed to add Collider, Entity {entity_id} is invalid?");
            }
            #[cfg(feature = "tracing")]
            Err(EntityNotSpawnedError::ValidButNotSpawned(..)) => {
                tracing::error!(target: "froglight_physics", "Failed to add Collider, Entity {entity_id} has not been spawned");
            }
            #[cfg(not(feature = "tracing"))]
            Err(_) => {}
        }
    }

    /// A [`System`] that updates [`EntityCollisions`] and [`CollidingWith`]
    /// based on entity [`Collider`]s.
    ///
    /// # Note
    ///
    /// This [`System`] is not scheduled by default! You must add it manually!
    pub fn update_entity_collisions(
        mut query: Query<(Entity, &Collider, &mut CollidingWith)>,
        mut collisions: ResMut<EntityCollisions>,
    ) {
        let mut iter = query.iter_combinations_mut();

        while let Some(
            [
                (entity_a, collider_a, mut colliding_with_a),
                (entity_b, collider_b, mut colliding_with_b),
            ],
        ) = iter.fetch_next()
        {
            if collider_a.intersects(collider_b) {
                if collisions.push_pair(entity_a, entity_b) {
                    colliding_with_a.insert(entity_b);
                    colliding_with_b.insert(entity_a);
                }
            } else if collisions.remove_pair(entity_a, entity_b) {
                colliding_with_a.remove(&entity_b);
                colliding_with_b.remove(&entity_a);
            }
        }
    }

    /// A [`System`] that updates last-tick physics [`Component`]s.
    ///
    /// # Note
    ///
    /// This [`System`] is not scheduled by default! You must add it manually!
    ///
    /// Updates:
    /// - [`Acceleration`] -> [`PrevAcceleration`]
    /// - [`Position`] -> [`PrevPosition`]
    /// - [`Rotation`] -> [`PrevRotation`]
    /// - [`Velocity`] -> [`PrevVelocity`]
    /// - [`Collider`] -> [`PrevCollider`]
    #[expect(clippy::missing_panics_doc, reason = "Components are dense, so `unwrap` is ok.")]
    pub fn update_prev_components(
        mut accel: Query<(&Acceleration, &mut PrevAcceleration)>,
        mut pos: Query<(&Position, &mut PrevPosition)>,
        mut rot: Query<(&Rotation, &mut PrevRotation)>,
        mut vel: Query<(&Velocity, &mut PrevVelocity)>,
        mut col: Query<(&Collider, &mut PrevCollider)>,
    ) {
        ComputeTaskPool::get().scope::<_, ()>(|scope| {
            scope.spawn(async {
                for (accel, prev) in accel.contiguous_iter_mut().unwrap() {
                    for (accel, prev) in accel.iter().zip(prev) {
                        *prev = PrevAcceleration::new_accel(*accel);
                    }
                }
            });
            scope.spawn(async {
                for (pos, prev) in pos.contiguous_iter_mut().unwrap() {
                    for (pos, prev) in pos.iter().zip(prev) {
                        *prev = PrevPosition::new_pos(*pos);
                    }
                }
            });
            scope.spawn(async {
                for (rot, prev) in rot.contiguous_iter_mut().unwrap() {
                    for (rot, prev) in rot.iter().zip(prev) {
                        *prev = PrevRotation::new_rot(*rot);
                    }
                }
            });
            scope.spawn(async {
                for (vel, prev) in vel.contiguous_iter_mut().unwrap() {
                    for (vel, prev) in vel.iter().zip(prev) {
                        *prev = PrevVelocity::new_vel(*vel);
                    }
                }
            });
            scope.spawn(async {
                for (col, prev) in col.contiguous_iter_mut().unwrap() {
                    for (col, prev) in col.iter().zip(prev) {
                        *prev = PrevCollider::new_col(*col);
                    }
                }
            });
        });
    }
}
