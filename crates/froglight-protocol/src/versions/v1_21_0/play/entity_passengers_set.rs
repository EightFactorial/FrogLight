use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct EntityPassengersSetPacket {
    pub entity_id: EntityId,
    pub passengers: Vec<EntityId>,
}
