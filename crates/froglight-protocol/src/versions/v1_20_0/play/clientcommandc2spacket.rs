use froglight_macros::FrogReadWrite;

use crate::{common::EntityId, packet::ClientPlayerCommand};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0])]
pub struct ClientCommandC2SPacket {
    pub entity_id: EntityId,
    pub mode: ClientPlayerCommand,
    #[frog(var)]
    pub mount_jump_height: u32,
}
