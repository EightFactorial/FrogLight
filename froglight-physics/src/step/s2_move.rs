//! TODO

use crate::step::{ChunkQuery, CollidingQuery, EntityQuery, PhysicsInput};

/// Perform the movement step of the physics simulation.
pub fn move_step<E: EntityQuery, C: CollidingQuery, W: ChunkQuery>(
    _input: &mut PhysicsInput<E, C, W>,
) {
}
