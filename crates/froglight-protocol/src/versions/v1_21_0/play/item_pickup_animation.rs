use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ItemPickupAnimationPacket {
    pub collected_id: EntityId,
    pub collector_id: EntityId,
    #[frog(var)]
    pub pickup_count: u32,
}
