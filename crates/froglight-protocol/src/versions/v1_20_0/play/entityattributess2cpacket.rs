use froglight_macros::FrogReadWrite;

use crate::common::{EntityId, UnsizedByteBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [42, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntityAttributesS2CPacket {
    pub entity_id: EntityId,
    // TODO: Implement attributes
    pub data: UnsizedByteBuffer,
    // pub attributes: Vec<EntityAttribute>,
}
