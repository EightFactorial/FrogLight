use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [32, 5, 0, 1, 2, 3, 4])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntityPassengersSetS2CPacket {
    pub id: EntityId,
    pub passenger_ids: Vec<EntityId>,
}
