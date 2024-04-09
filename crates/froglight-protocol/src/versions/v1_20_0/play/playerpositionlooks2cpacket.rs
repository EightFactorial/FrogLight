use froglight_macros::FrogReadWrite;
use glam::DVec3;

use crate::packet::RelativePositionFlags;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct PlayerPositionLookS2CPacket {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: RelativePositionFlags,
    #[frog(var)]
    pub teleport_id: u32,
}
