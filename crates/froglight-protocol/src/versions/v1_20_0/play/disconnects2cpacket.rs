use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Default, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct DisconnectS2CPacket {
    pub reason: Value,
}
