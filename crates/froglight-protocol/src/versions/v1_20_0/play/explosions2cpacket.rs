use froglight_macros::FrogReadWrite;
use glam::{DVec3, Vec3};

use crate::common::BlockPosition;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ExplosionS2CPacket {
    pub position: DVec3,
    pub radius: f32,
    pub affected_blocks: Vec<BlockPosition>,
    pub player_velocity: Vec3,
}
