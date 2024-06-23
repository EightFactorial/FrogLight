use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct TitlePacket {
    // TODO: Text
    pub title: Value,
}
