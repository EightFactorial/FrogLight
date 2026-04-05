//! TODO

use crate::step::{ChunkQuery, EntityQuery, PhysicsInput};

/// Perform the fluid step of the physics simulation.
pub fn fluid_step<'s, E: EntityQuery<'s>, W: ChunkQuery>(_input: &mut PhysicsInput<'s, E, W>) {}
