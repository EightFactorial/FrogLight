use froglight_components::entity::EntityId;
use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct DeathMessagePacket {
    pub entity_id: EntityId,
    // TODO: Text
    pub message: Value,
}
