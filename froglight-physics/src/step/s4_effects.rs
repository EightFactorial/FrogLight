//! TODO

use crate::step::{ChunkQuery, EntityQuery, PhysicsInput};

/// Perform the world effect step of the physics simulation.
pub fn effect_step<'s, E: EntityQuery<'s>, W: ChunkQuery>(_input: &mut PhysicsInput<'s, E, W>) {}
