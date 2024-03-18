use bevy_math::DVec3;
use froglight_macros::FrogReadWrite;

use crate::common::RelativePositionFlags;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerPositionLookS2CPacket {
    pub position: DVec3,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: RelativePositionFlags,
    #[frog(var)]
    pub teleport_id: u32,
}
