use bevy_math::U16Vec3;
use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntityS2CPacketRotateAndMoveRelative {
    pub id: EntityId,
    pub delta: U16Vec3,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
