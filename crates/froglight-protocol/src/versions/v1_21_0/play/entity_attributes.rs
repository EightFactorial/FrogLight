use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [42, 0])]
pub struct EntityAttributesPacket {
    pub entity_id: EntityId,
    // TODO: Implement AttributeData
    pub data: UnsizedBuffer,
}
