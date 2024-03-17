use froglight_macros::FrogReadWrite;

use crate::common::ServerStatus;

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct QueryResponseS2CPacket(pub ServerStatus);
