use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, FrogReadWrite)]
pub struct DisconnectS2CPacket {
    pub reason: Value,
}
