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
pub fn step<E: EntityQuery, W: ChunkQuery>(mut input: PhysicsInput<E, W>) {
    s1_fluid::fluid_step(&mut input);
    s2_move::move_step(&mut input);
    s3_travel::travel_step(&mut input);
    s4_effects::effect_step(&mut input);
}

// -------------------------------------------------------------------------------------------------

/// The input for the physics [`step`] function.
pub struct PhysicsInput<'a, E: EntityQuery, W: ChunkQuery> {
    /// The entity to perform the physics step for.
    target_id: E::ID,
    /// The target entity's [`PhysicsMut`].
    target: PhysicsMut<'a>,

    /// Access to entity data.
    entities: E,
    /// Access to chunk data.
    world: W,
}

impl<'a, E: EntityQuery, W: ChunkQuery> PhysicsInput<'a, E, W> {
    /// Create a new [`PhysicsInput`] for the given target.
    #[inline]
    #[must_use]
    pub const fn new(target_id: E::ID, target: PhysicsMut<'a>, entities: E, world: W) -> Self {
        Self { target_id, target, entities, world }
    }

    /// Get the target entity's [`PhysicsRef`].
    #[must_use]
    pub const fn target(&self) -> PhysicsRef<'_> { self.target.as_ref() }

    /// Get the target entity's [`PhysicsMut`].
    #[must_use]
    pub const fn target_mut(&mut self) -> PhysicsMut<'_> { self.target.reborrow() }

    /// Get the [`PhysicsRef`] for an entity, if it exists.
    #[must_use]
    pub fn get_entity(&self, entity: E::ID) -> Option<PhysicsRef<'_>> {
        if entity == self.target_id {
            Some(self.target.as_ref())
        } else {
            self.entities.get_entity(entity)
        }
    }

    /// Get the [`PhysicsMut`] for an entity, if it exists.
    #[must_use]
    pub fn get_entity_mut(&mut self, entity: E::ID) -> Option<PhysicsMut<'_>> {
        if entity == self.target_id {
            Some(self.target.reborrow())
        } else {
            self.entities.get_entity_mut(entity)
        }
    }

    /// Get a guard for a chunk, if it exists.
    #[inline]
    #[must_use]
    pub fn get_chunk(&self, chunk: &ChunkPos) -> Option<W::Guard> { self.world.get_chunk(chunk) }

    /// Get all parts of the input as a tuple.
    ///
    /// # SAFETY
    ///
    /// The caller must ensure that the target entity's [`PhysicsMut`] is
    /// *never* accessed through `E`.
    #[inline]
    #[must_use]
    pub const unsafe fn as_mut_parts(&mut self) -> (E::ID, PhysicsMut<'_>, &mut E, &mut W) {
        let Self { target_id, target, entities, world } = self;
        (*target_id, target.reborrow(), entities, world)
    }
}

// -------------------------------------------------------------------------------------------------

/// A type that can query for entities' physics components.
pub trait EntityQuery {
    /// An identifier for an entity.
    type ID: Copy + Eq;

    /// Get the physics components for an entity, if it exists.
    fn get_entity(&self, entity: Self::ID) -> Option<PhysicsRef<'_>>;

    /// Muatably get the physics components for an entity, if it exists.
    fn get_entity_mut(&mut self, entity: Self::ID) -> Option<PhysicsMut<'_>>;
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
