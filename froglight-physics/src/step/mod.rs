//! TODO

use froglight_world::{chunk::NaiveChunk, prelude::ChunkPos};

use crate::prelude::{PhysicsMut, PhysicsRef};

pub mod s1_fluid;
pub mod s2_move;
pub mod s3_travel;
pub mod s4_effects;

/// Perform a physics step.
///
/// Just a wrapper for calling the individual step functions in order.
pub fn step<E: EntityQuery<ID>, C: CollidingQuery<ID>, W: ChunkQuery, ID: Copy + Eq>(
    mut input: PhysicsInput<E, C, W, ID>,
) {
    s1_fluid::fluid_step(&mut input);
    s2_move::move_step(&mut input);
    s3_travel::travel_step(&mut input);
    s4_effects::effect_step(&mut input);
}

// -------------------------------------------------------------------------------------------------

/// The input for the physics [`step`] function.
pub struct PhysicsInput<E: EntityQuery<ID>, C: CollidingQuery<ID>, W: ChunkQuery, ID: Copy + Eq> {
    /// The entity to perform the physics step for.
    target_id: ID,
    /// Access to entity data.
    entities: E,
    /// Access to colliding entities.
    colliding: C,
    /// Access to chunk data.
    world: W,
}

impl<E: EntityQuery<ID>, C: CollidingQuery<ID>, W: ChunkQuery, ID: Copy + Eq>
    PhysicsInput<E, C, W, ID>
{
    /// Create a new [`PhysicsInput`] for the given target.
    #[inline]
    #[must_use]
    pub const fn new(target_id: ID, entities: E, colliding: C, world: W) -> Self {
        Self { target_id, entities, colliding, world }
    }

    /// Get the target entity's ID.
    #[inline]
    #[must_use]
    pub const fn target(&self) -> ID { self.target_id }

    /// Get the target entity's [`PhysicsRef`].
    #[inline]
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't ever panic")]
    pub fn target_ref(&self) -> PhysicsRef<'_> { self.get_entity(self.target_id).unwrap() }

    /// Get the target entity's [`PhysicsMut`].
    #[inline]
    #[must_use]
    #[expect(clippy::missing_panics_doc, reason = "Shouldn't ever panic")]
    pub fn target_mut(&mut self) -> PhysicsMut<'_> { self.get_entity_mut(self.target_id).unwrap() }

    /// Get the [`PhysicsRef`] for an entity, if it exists.
    #[inline]
    #[must_use]
    pub fn get_entity(&self, entity: ID) -> Option<PhysicsRef<'_>> {
        self.entities.get_entity(entity)
    }

    /// Get the [`PhysicsMut`] for an entity, if it exists.
    #[inline]
    #[must_use]
    pub fn get_entity_mut(&mut self, entity: ID) -> Option<PhysicsMut<'_>> {
        self.entities.get_entity_mut(entity)
    }

    /// Get the entities that are colliding with the given entity, if any.
    #[inline]
    #[must_use]
    pub fn get_colliding(&self, entity: ID) -> Option<impl Iterator<Item = ID> + '_> {
        self.colliding.get_colliding(entity)
    }

    /// Get a guard for a chunk, if it exists.
    #[inline]
    #[must_use]
    pub fn get_chunk(&self, chunk: &ChunkPos) -> Option<W::Guard> { self.world.get_chunk(chunk) }

    /// Get all parts of the input as a tuple.
    #[inline]
    #[must_use]
    pub const fn as_mut_parts(&mut self) -> (ID, &mut E, &mut C, &mut W) {
        let Self { target_id, entities, colliding, world } = self;
        (*target_id, entities, colliding, world)
    }
}

// -------------------------------------------------------------------------------------------------

/// A type that can query for entities' physics components.
pub trait EntityQuery<ID> {
    /// Get the physics components for an entity, if it exists.
    fn get_entity(&self, entity: ID) -> Option<PhysicsRef<'_>>;

    /// Muatably get the physics components for an entity, if it exists.
    fn get_entity_mut(&mut self, entity: ID) -> Option<PhysicsMut<'_>>;
}

/// A type that can query for colliding entities.
pub trait CollidingQuery<ID> {
    /// Get the entities that are colliding with the given entity, if any.
    fn get_colliding(&self, entity: ID) -> Option<impl Iterator<Item = ID> + '_>;
}

/// A type that can query for chunk data.
pub trait ChunkQuery {
    /// A guard for a chunk.
    type Guard: ChunkGuard;

    /// Get a guard for a chunk, if it exists.
    fn get_chunk(&self, chunk: &ChunkPos) -> Option<Self::Guard>;
}

/// A type acting as a guard for a chunk, providing access to its data.
pub trait ChunkGuard {
    /// Get the chunk as a [`NaiveChunk`].
    fn naive(&self) -> &NaiveChunk;
}
