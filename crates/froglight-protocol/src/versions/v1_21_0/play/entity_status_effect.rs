use froglight_components::entity::EntityId;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct EntityStatusEffectPacket {
    pub entity_id: EntityId,
    #[frog(var)]
    pub effect_id: u32,
    #[frog(var)]
    pub amplifier: u32,
    #[frog(var)]
    pub duration: u32,
    // TODO: Implement StatusEffectFlags
    pub flags: u8,
}
