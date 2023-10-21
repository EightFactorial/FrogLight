use mc_rs_macros::Transcode;

use crate::types::{UnsizedByteBuffer, Vec3};

// TODO: Parse this packet
// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundParticlePacket {
    #[var]
    pub particle_id: u32,
    pub override_limit: bool,
    pub position: Vec3,
    pub x_distance: f32,
    pub y_distance: f32,
    pub z_distance: f32,
    pub max_speed: f32,
    pub particle_count: u32,
    pub data: UnsizedByteBuffer,
}
