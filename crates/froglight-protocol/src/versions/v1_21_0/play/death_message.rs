use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;
use simdnbt::owned::NbtTag;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct DeathMessagePacket {
    pub entity_id: EntityId,
    pub message: NbtTag,
}
