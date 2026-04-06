//! TODO

use crate::step::{ChunkQuery, CollidingQuery, EntityQuery, PhysicsInput};

/// Perform the fluid step of the physics simulation.
pub fn fluid_step<E: EntityQuery, C: CollidingQuery, W: ChunkQuery>(
    _input: &mut PhysicsInput<E, C, W>,
) {
}
