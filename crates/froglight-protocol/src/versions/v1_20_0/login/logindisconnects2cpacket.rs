use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct LoginDisconnectS2CPacket {
    // TODO: FormattedText
    pub reason: Value,
}
