use froglight_macros::FrogReadWrite;
use glam::DVec3;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ExplosionPacket {
    pub position: DVec3,
    pub radius: f32,
    pub blocks: Vec<[i8; 3]>,
    pub player_velocity: DVec3,
    pub explosion_data: UnsizedBuffer,
}
