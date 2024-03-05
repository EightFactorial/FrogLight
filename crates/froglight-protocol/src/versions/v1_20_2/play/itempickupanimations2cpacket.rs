use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ItemPickupAnimationS2CPacket {
    pub entity_id: (),
    pub collector_entity_id: (),
    pub stack_amount: (),
}
