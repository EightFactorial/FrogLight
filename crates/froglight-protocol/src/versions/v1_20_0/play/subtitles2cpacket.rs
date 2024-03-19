use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct SubtitleS2CPacket(pub Value);
