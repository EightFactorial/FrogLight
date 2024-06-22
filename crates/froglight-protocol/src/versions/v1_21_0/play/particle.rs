use froglight_macros::FrogReadWrite;
use glam::DVec3;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ParticlePacket {
    #[frog(var)]
    pub particle_id: u32,
    pub override_limit: bool,
    pub position: DVec3,
    pub max_speed: f32,
    pub particle_count: f32,
    // TODO: Implement ParticleData
    pub particle_data: UnsizedBuffer,
}
