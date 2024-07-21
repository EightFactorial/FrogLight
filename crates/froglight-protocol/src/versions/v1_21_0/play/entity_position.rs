use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;
use glam::DVec3;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])]
pub struct EntityPositionPacket {
    pub entity_id: EntityId,
    pub position: DVec3,
    pub yaw: u8,
    pub pitch: u8,
    pub on_ground: bool,
}
