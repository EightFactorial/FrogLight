use froglight_macros::FrogReadWrite;
use serde_json::Value;

use crate::common::EntityId;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct DeathMessageS2CPacket {
    pub id: EntityId,
    pub message: Value,
}
