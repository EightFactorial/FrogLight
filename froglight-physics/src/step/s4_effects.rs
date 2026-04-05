//! TODO

use crate::step::{ChunkQuery, EntityQuery, PhysicsInput};

/// Perform the world effect step of the physics simulation.
pub fn effect_step<E: EntityQuery, W: ChunkQuery>(_input: &mut PhysicsInput<E, W>) {}
