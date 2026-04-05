//! TODO

use crate::step::{ChunkQuery, EntityQuery, PhysicsInput};

/// Perform the travel step of the physics simulation.
pub fn travel_step<'s, E: EntityQuery<'s>, W: ChunkQuery>(_input: &mut PhysicsInput<'s, E, W>) {}
