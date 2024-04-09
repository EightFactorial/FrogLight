use froglight_macros::FrogReadWrite;
use glam::I16Vec3;

use crate::common::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0])]
pub struct EntityVelocityUpdateS2CPacket {
    pub id: EntityId,
    pub velocity: I16Vec3,
}
