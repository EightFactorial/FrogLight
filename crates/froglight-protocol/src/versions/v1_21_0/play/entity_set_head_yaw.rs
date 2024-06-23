use froglight_components::entity::EntityId;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct EntitySetHeadYawPacket {
    pub entity_id: EntityId,
    pub head_yaw: i8,
}
