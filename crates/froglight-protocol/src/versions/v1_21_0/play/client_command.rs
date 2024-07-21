use froglight_common::EntityId;
use froglight_macros::FrogReadWrite;

use crate::packet::ClientPlayerCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0])]
pub struct ClientCommandPacket {
    pub entity_id: EntityId,
    pub command: ClientPlayerCommand,
    #[frog(var)]
    pub jump_height: u32,
}
