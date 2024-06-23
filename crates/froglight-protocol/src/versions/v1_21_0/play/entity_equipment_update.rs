use froglight_components::entity::EntityId;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [4, 0, 1, 153, 6, 1, 10, 0, 0, 3, 0, 6, 68, 97, 109, 97, 103, 101, 0, 0, 0, 0, 0])]
pub struct EntityEquipmentUpdatePacket {
    pub entity_id: EntityId,
    // TODO: Implement EquipmentData
    pub equipment: UnsizedBuffer,
}
