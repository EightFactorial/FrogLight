//! TODO

use crate::step::{ChunkQuery, CollidingQuery, EntityQuery, PhysicsInput};

/// Perform the travel step of the physics simulation.
pub fn travel_step<E: EntityQuery, C: CollidingQuery, W: ChunkQuery>(
    _input: &mut PhysicsInput<E, C, W>,
) {
}
