use bevy_math::DVec3;
use froglight_macros::FrogReadWrite;

use crate::common::{EntityId, EntityUuid};

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerSpawnS2CPacket {
    pub id: EntityId,
    pub uuid: EntityUuid,
    pub position: DVec3,
    pub yaw: i8,
    pub pitch: i8,
}
