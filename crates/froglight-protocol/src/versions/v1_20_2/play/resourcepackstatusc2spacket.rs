use froglight_macros::FrogReadWrite;
use uuid::Uuid;

use crate::common::ResourcePackAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ResourcePackStatusC2SPacket {
    pub id: Uuid,
    pub status: ResourcePackAction,
}
