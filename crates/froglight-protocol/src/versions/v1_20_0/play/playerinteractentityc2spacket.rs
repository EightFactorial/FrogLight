use froglight_macros::FrogReadWrite;

use crate::{common::EntityId, packet::PlayerInteraction};

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 1, 0])]
pub struct PlayerInteractEntityC2SPacket {
    pub entity_id: EntityId,
    pub interaction: PlayerInteraction,
    pub player_sneaking: bool,
}
