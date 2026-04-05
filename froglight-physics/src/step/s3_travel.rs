//! TODO

use crate::step::{ChunkQuery, EntityQuery, PhysicsInput};

/// Perform the travel step of the physics simulation.
pub fn travel_step<E: EntityQuery, W: ChunkQuery>(_input: &mut PhysicsInput<E, W>) {}
