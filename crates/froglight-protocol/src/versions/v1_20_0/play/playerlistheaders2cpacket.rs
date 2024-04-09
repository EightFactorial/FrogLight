use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
pub struct PlayerListHeaderS2CPacket {
    pub header: Value,
    pub footer: Value,
}
