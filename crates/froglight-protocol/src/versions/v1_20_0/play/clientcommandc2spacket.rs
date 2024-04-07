use froglight_macros::FrogReadWrite;

use crate::common::{ClientPlayerCommand, EntityId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ClientCommandC2SPacket {
    pub entity_id: EntityId,
    pub mode: ClientPlayerCommand,
    #[frog(var)]
    pub mount_jump_height: u32,
}
