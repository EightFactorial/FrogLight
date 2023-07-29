use bevy_math::Vec3;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundVehicleMovePacket {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}
