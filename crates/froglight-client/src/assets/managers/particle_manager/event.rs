use bevy::{math::DVec3, prelude::*};
use froglight_network::common::ResourceKey;
use simdnbt::owned::Nbt;

/// A particle event.
///
/// Spawns a particle, if it exists
/// in the [`ParticleManager`](super::ParticleManager).
#[derive(Debug, Clone, PartialEq, Event)]
pub struct ParticleEvent {
    /// The particle key.
    pub key: ResourceKey,
    /// The particle position.
    pub position: DVec3,
    /// The maximum particle offset.
    pub max_offset: Vec3,
    /// The maximum particle speed.
    pub max_speed: f32,
    /// The number of particles to spawn.
    pub count: u32,
    /// Optional NBT data.
    pub data: Nbt,
}
