use froglight_macros::FrogReadWrite;
use glam::DVec3;

use crate::common::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])]
pub struct EntityPositionS2CPacket {
    pub id: EntityId,
    pub position: DVec3,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
