//! TODO

use crate::step::{ChunkQuery, CollidingQuery, EntityQuery, PhysicsInput};

/// Perform the world effect step of the physics simulation.
pub fn effect_step<E: EntityQuery<ID>, C: CollidingQuery<ID>, W: ChunkQuery, ID: Copy + Eq>(
    _input: &mut PhysicsInput<E, C, W, ID>,
) {
}
