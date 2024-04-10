use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, FrogReadWrite)]
pub struct TitleS2CPacket {
    pub title: Value,
}
