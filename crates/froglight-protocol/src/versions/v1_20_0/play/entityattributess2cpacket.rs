use froglight_macros::FrogReadWrite;

use crate::common::{EntityId, UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [42, 0])]
pub struct EntityAttributesS2CPacket {
    pub entity_id: EntityId,
    // TODO: Implement attributes
    pub data: UnsizedBuffer,
    // pub attributes: Vec<EntityAttribute>,
}
