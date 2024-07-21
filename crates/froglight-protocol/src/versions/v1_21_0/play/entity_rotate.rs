use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0])]
pub struct EntityRotatePacket {
    pub entity_id: EntityId,
    pub yaw: i8,
    pub pitch: i8,
    pub on_ground: bool,
}
