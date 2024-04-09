use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0])]
pub struct EntitySetHeadYawS2CPacket {
    pub entity: EntityId,
    pub head_yaw: i8,
}
