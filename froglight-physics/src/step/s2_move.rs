//! TODO

use crate::step::{ChunkQuery, EntityQuery, PhysicsInput};

/// Perform the movement step of the physics simulation.
pub fn move_step<E: EntityQuery, W: ChunkQuery>(_input: &mut PhysicsInput<E, W>) {}
