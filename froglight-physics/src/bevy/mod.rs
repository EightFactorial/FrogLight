//! TODO

use alloc::vec::Vec;

use bevy_app::{App, Plugin};
#[cfg(feature = "tracing")]
use bevy_ecs::entity::EntityNotSpawnedError;
use bevy_ecs::{entity::UniqueEntityArray, prelude::*, world::DeferredWorld};
use bevy_tasks::ComputeTaskPool;
use froglight_entity::{bevy::EntityBundleEvent, prelude::EntityBundle};
use froglight_instance::prelude::SessionInstance;
use parking_lot::Mutex;

use crate::prelude::*;

pub mod colliding;
pub mod collision_cache;

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
                    // Insert `Collider` and preserve `Position` and `Rotation` (added if missing).
                    // Forcefully overwrite `Velocity` and `Acceleration` to `ZERO`.
                    let mut collider = Collider::new_entity(*bundle.metadata().aabb());
                    if let Some(pos) = entity.get::<Position>() {
                        collider.set_center(pos.to_vec3a());
                    }

                    let mut commands = commands.entity(entity_id);
                    commands.insert((collider, Velocity::ZERO, Acceleration::ZERO));
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
                tracing::error!(target: "froglight_physics", "Failed to add Collider, Entity {entity_id} does not exist?");
            }
            #[cfg(not(feature = "tracing"))]
            Err(_) => {}
        }
    }

    /// A [`System`] that updates [`Collider`]s based on entity [`Position`]s.
    ///
    /// # Note
    ///
    /// This [`System`] is not scheduled by default! You must add it manually!
    pub fn update_colliders(colliders: Query<(&Position, &mut Collider)>) {
        colliders.par_iter_inner().for_each(|(pos, mut collider)| {
            collider.set_center(pos.to_vec3a());
        });
    }

    /// A [`System`] that updates [`EntityCollisions`] and [`CollidingWith`]
    /// based on entity [`Collider`]s.
    ///
    /// # Note
    ///
    /// This [`System`] is not scheduled by default! You must add it manually!
    pub fn update_collisions(
        instances: Query<(Entity, &SessionInstance)>,
        mut colliders: Query<(Entity, &Collider, &mut CollidingWith)>,
        mut collisions: ResMut<EntityCollisions>,
        mut cache: Local<Mutex<Vec<UniqueEntityArray<2>>>>,
    ) {
        let collider_lens = colliders.transmute_lens::<(Entity, &Collider)>();
        let collider_lens = collider_lens.query_inner();

        // Calculate all collisions in parallel.
        instances.par_iter().for_each(|(_entity, instance)| {
            #[cfg(feature = "tracing")]
            #[allow(clippy::used_underscore_binding, reason = "Used for tracing")]
            let _span = tracing::info_span!(target: "froglight_physics", "par_update_collisions", instance = %_entity).entered();

            let mut entities = Vec::<&Entity>::with_capacity(instance.iter_entity().len());
            entities.extend(instance.iter_entity());

            for (start, a) in entities.iter().enumerate() {
                for b in entities.iter().skip(start + 1) {
                    // Skip checking collisions with itself.
                    if a == b {
                        continue;
                    }

                    let (a, b) = (**a, **b);
                    if let (Ok((_, collider_a)), Ok((_, collider_b))) =
                        (collider_lens.get(a), collider_lens.get(b))
                        && collider_a.intersects( collider_b)
                    {
                        // SAFETY: Already checked that `a` and `b` are not equal.
                        cache
                            .lock()
                            .push(unsafe { UniqueEntityArray::from_array_unchecked([a, b]) });
                    }
                }
            }
        });

        // Clear all existing collisions.
        collisions.clear();
        for (_, _, mut colliding_with) in colliders.iter_mut() {
            colliding_with.clear();
        }

        // Insert all new collisions.
        for pair in cache.get_mut().drain(..) {
            let [a, b] = pair.into_inner();
            if collisions.push_pair(a, b)
                && let Ok([(.., mut colliding_with_a), (.., mut colliding_with_b)]) =
                    colliders.get_many_unique_mut(pair)
            {
                colliding_with_a.insert(a);
                colliding_with_b.insert(b);
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

            for (col, prev) in col.contiguous_iter_mut().unwrap() {
                for (col, prev) in col.iter().zip(prev) {
                    *prev = PrevCollider::new_col(*col);
                }
            }
        });
    }
}
