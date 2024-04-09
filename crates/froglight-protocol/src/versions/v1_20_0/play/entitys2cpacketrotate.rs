use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0])]
pub struct EntityS2CPacketRotate {
    pub id: EntityId,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
