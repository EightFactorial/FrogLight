use froglight_macros::FrogReadWrite;

use crate::common::{EntityId, ResourceKey};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [8, 5, 77, 67, 45, 82, 83])]
pub struct RemoveEntityStatusEffectS2CPacket {
    pub entity_id: EntityId,
    pub effect_type: ResourceKey,
}
