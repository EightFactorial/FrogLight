use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct RemoveEntityStatusEffectPacket {
    pub entity_id: EntityId,
    #[frog(var)]
    pub effect_id: u32,
}
