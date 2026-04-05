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
pub struct PhysicsInput<E: EntityQuery, W: ChunkQuery> {
    /// The entity to perform the physics step for.
    pub target: E::ID,
    /// Access to entity data.
    pub entities: E,
    /// Access to chunk data.
    pub world: W,
}

/// A type that can query for entities' physics components.
pub trait EntityQuery {
    /// An identifier for an entity.
    type ID;

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
