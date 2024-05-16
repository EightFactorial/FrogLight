use bevy::prelude::*;
use froglight_network::common::ResourceKey;

/// A particle event.
///
/// Spawns a particle at the given position
/// with the given [`ResourceKey`], if it exists
/// in the [`ParticleManager`](super::ParticleManager).
#[derive(Debug, Clone, PartialEq, Event)]
pub struct ParticleEvent {
    /// The particle key.
    pub key: ResourceKey,
    /// The particle position.
    pub position: Vec3,
}
