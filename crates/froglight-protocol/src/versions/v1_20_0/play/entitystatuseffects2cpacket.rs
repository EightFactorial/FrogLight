use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

use crate::common::{EntityId, ResourceKey};

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntityStatusEffectS2CPacket {
    pub entity_id: EntityId,
    pub effect: ResourceKey,
    pub amplifier: u8,
    #[frog(var)]
    pub duration: u32,
    pub flags: u8,
    pub data: Option<Nbt>,
}
