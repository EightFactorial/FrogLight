use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct QueryEntityNbtPacket {
    #[frog(var)]
    pub id: u32,
    pub entity_id: EntityId,
}
