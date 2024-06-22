use froglight_components::entity::EntityId;
use froglight_macros::FrogReadWrite;
use glam::I16Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct EntityRotateAndMoveRelativePacket {
    pub entity_id: EntityId,
    pub delta: I16Vec3,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
