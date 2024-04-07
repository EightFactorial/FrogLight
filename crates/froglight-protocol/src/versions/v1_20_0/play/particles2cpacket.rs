use bevy_math::{DVec3, Vec3};
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ParticleS2CPacket {
    #[frog(var)]
    pub particle_id: u32,
    pub override_limit: bool,
    pub position: DVec3,
    pub distance: Vec3,
    pub max_speed: f32,
    pub particle_count: u32,
    pub data: UnsizedByteBuffer,
}
