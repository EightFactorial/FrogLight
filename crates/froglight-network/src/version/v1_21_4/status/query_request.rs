#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_macros::FrogBuf;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogBuf)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, Clone, PartialEq, Hash))]
pub struct QueryRequestPacket;
