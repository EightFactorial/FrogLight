use froglight_components::entity::EntityId;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ProjectilePowerPacket {
    pub entity_id: EntityId,
    pub power: f64,
}
