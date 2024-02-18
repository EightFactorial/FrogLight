use bevy::prelude::*;

/// The [`Plugin`] for the [`froglight-physics`](crate) crate.
///
/// Adds entity physics and collision.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, _app: &mut App) {}
}
