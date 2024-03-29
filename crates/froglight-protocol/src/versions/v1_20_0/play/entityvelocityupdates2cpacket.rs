use bevy_math::I16Vec3;
use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntityVelocityUpdateS2CPacket {
    pub id: EntityId,
    pub velocity: I16Vec3,
}
