use crate::types::Vec3;
use mc_rs_macros::Transcode;

#[derive(Debug, Default, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ClientboundVehicleMovePacket {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}
