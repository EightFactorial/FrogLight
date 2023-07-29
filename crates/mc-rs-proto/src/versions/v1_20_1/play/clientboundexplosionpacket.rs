use bevy_math::Vec3;
use mc_rs_macros::Transcode;

use crate::types::position::BlockPos;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundExplosionPacket {
    pub position: Vec3,
    pub power: f32,
    pub block_list: Vec<BlockPos>,
    pub knockback_x: f32,
    pub knockback_y: f32,
    pub knockback_z: f32,
}
