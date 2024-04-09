use froglight_macros::FrogReadWrite;
use serde_json::Value;

use crate::common::EntityId;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct DeathMessageS2CPacket {
    pub id: EntityId,
    pub message: Value,
}
