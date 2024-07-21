use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;

use crate::packet::PlayerInteraction;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 1, 0])]
pub struct PlayerInteractEntityPacket {
    pub entity_id: EntityId,
    pub interaction: PlayerInteraction,
    pub sneaking: bool,
}
