use compact_str::CompactString;
use froglight_components::entity::EntityId;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct UpdateCommandBlockMinecartPacket {
    pub entity_id: EntityId,
    pub command: CompactString,
    pub track_output: bool,
}
