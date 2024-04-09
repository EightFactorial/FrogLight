use froglight_macros::FrogReadWrite;
use glam::{DVec3, Vec3};

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
pub struct ParticleS2CPacket {
    #[frog(var)]
    pub particle_id: u32,
    pub override_limit: bool,
    pub position: DVec3,
    pub distance: Vec3,
    pub max_speed: f32,
    pub particle_count: u32,
    pub data: UnsizedBuffer,
}
