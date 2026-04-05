//! TODO

use crate::step::{ChunkQuery, EntityQuery, PhysicsInput};

/// Perform the fluid step of the physics simulation.
pub fn fluid_step<E: EntityQuery, W: ChunkQuery>(_input: &mut PhysicsInput<E, W>) {}
