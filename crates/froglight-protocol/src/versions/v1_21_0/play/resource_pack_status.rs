use froglight_macros::FrogReadWrite;
use uuid::Uuid;

use crate::packet::ResourcePackStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ResourcePackStatusPacket {
    pub resourcepack: Uuid,
    pub status: ResourcePackStatus,
}
