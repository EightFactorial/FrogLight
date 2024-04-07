use froglight_macros::FrogReadWrite;

use crate::common::{EntityId, InteractionAction};

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 1, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerInteractEntityC2SPacket {
    pub entity_id: EntityId,
    pub kind: InteractionAction,
    pub player_sneaking: bool,
}
