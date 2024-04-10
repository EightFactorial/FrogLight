use froglight_macros::FrogReadWrite;
use glam::DVec3;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct VehicleMoveS2CPacket {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
}
