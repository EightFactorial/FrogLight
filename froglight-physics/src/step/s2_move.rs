//! TODO

use crate::step::{ChunkQuery, EntityQuery, PhysicsInput};

/// Perform the movement step of the physics simulation.
pub fn move_step<'s, E: EntityQuery<'s>, W: ChunkQuery>(_input: &mut PhysicsInput<'s, E, W>) {}
