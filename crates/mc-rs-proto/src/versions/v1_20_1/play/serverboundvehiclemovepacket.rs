use crate::types::Vec3;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundVehicleMovePacket {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}
