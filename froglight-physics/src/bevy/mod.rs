//! TODO

use alloc::sync::Arc;

use arc_swap::Guard;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use froglight_common::prelude::*;
use froglight_entity::{bevy::EntityBundleEvent, entity::EntityAabb, prelude::*};
use froglight_world::{chunk::SharedChunk, prelude::*};

use crate::{
    prelude::*,
    query::{PhysicsMutItem, PhysicsMutReadOnlyItem},
    step::{ChunkGuard, ChunkQuery, EntityQuery, PhysicsInput},
};

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Transform>().register_type::<PreviousTransform>();
        app.register_type::<Velocity>().register_type::<PreviousVelocity>();
        app.register_type::<Acceleration>().register_type::<PreviousAcceleration>();
        app.register_type::<OnGround>().register_type::<PreviousOnGround>();
        app.register_type::<PhysicsState>().register_type::<PhysicsController>();

        app.add_observer(Self::entity_physics_observer);
    }
}

impl PhysicsPlugin {
    /// An [`Observer`] that listens for [`EntityBundleEvent`]s and inserts
    /// [`PhysicsState`]s.
    #[expect(clippy::type_complexity, reason = "Complex query")]
    pub fn entity_physics_observer(
        event: On<EntityBundleEvent>,
        query: Query<(), (With<EntityBundle>, With<EntityAabb>, Without<PhysicsState>)>,
        mut commands: Commands,
    ) {
        if query.contains(event.entity()) {
            commands.entity(event.entity()).insert(PhysicsState::default());
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Create a [`PhysicsInput`] for the given entity and call the provided
/// function with it.
///
/// # Panics
///
/// TODO
#[inline]
pub fn entity_as_input(
    entity: Entity,
    world: &mut World,
    f: impl FnMut(PhysicsInput<BevyQuery<'_>, WorldQuery<'_>>),
) {
    many_as_input::<1>([entity], world, f);
}

/// Create a [`PhysicsInput`] for the given entities and call the provided
/// function with them.
///
/// # Panics
///
/// TODO: Don't panic
pub fn many_as_input<const N: usize>(
    entities: [Entity; N],
    world: &mut World,
    mut f: impl FnMut(PhysicsInput<BevyQuery<'_>, WorldQuery<'_>>),
) {
    let mut physics = world.query::<PhysicsMut<'static>>();
    let mut chunks = world.query::<&SharedChunk>();
    let cell = world.as_unsafe_world_cell();

    // SAFETY: None of the queries can access any other's components.
    // SAFETY: No archetype changes can occur through `PhysicsInput`.
    unsafe {
        let mut physics = physics.query_unchecked(cell);
        let mut chunks = chunks.query_unchecked(cell);

        for entity in entities {
            let instance_id =
                cell.get_entity(entity).unwrap().get::<EntityOfInstance>().unwrap().entity();
            let instance =
                cell.get_entity(instance_id).unwrap().get::<WorldInstanceChunks>().unwrap();

            f(PhysicsInput {
                target: entity,
                entities: BevyQuery(physics.reborrow()),
                world: WorldQuery { instance, query: chunks.reborrow() },
            });
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// An [`EntityQuery`] implementation for Bevy.
pub struct BevyQuery<'s>(Query<'s, 's, PhysicsMut<'static>, ()>);

impl<'s> EntityQuery<'s> for BevyQuery<'s> {
    type ID = Entity;

    #[inline]
    fn get_entity(&self, entity: Self::ID) -> Option<PhysicsMutReadOnlyItem<'_, 's, 'static>> {
        self.0.get(entity).ok()
    }

    #[inline]
    fn get_entity_mut(&mut self, entity: Self::ID) -> Option<PhysicsMutItem<'_, 's, 'static>> {
        self.0.get_mut(entity).ok()
    }
}

/// A [`ChunkQuery`] implementation for Bevy.
pub struct WorldQuery<'s> {
    instance: &'s WorldInstanceChunks,
    query: Query<'s, 's, &'static SharedChunk, ()>,
}

impl ChunkQuery for WorldQuery<'_> {
    type Guard = Guard<Arc<Chunk>>;

    fn get_chunk(&self, chunk: &ChunkPos) -> Option<Self::Guard> {
        self.instance.get(chunk).and_then(|entity| self.query.get(entity).ok().map(|c| c.load()))
    }
}
impl ChunkGuard for Guard<Arc<Chunk>> {
    #[inline]
    fn naive(&self) -> &NaiveChunk { self.as_naive() }
}
