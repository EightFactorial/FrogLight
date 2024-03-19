use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerListHeaderS2CPacket {
    pub header: Value,
    pub footer: Value,
}
