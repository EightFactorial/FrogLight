use froglight_macros::FrogReadWrite;

use crate::common::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [32, 16, 64])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ItemPickupAnimationS2CPacket {
    pub entity_id: EntityId,
    pub collector_entity_id: EntityId,
    #[frog(var)]
    pub stack_amount: u32,
}
