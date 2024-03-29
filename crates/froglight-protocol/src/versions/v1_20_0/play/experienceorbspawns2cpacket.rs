use bevy_math::DVec3;
use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ExperienceOrbSpawnS2CPacket {
    pub id: EntityId,
    pub position: DVec3,
    pub experience: u16,
}
