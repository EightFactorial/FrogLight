#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FrogBuf, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub struct PingResultPacket {
    pub pong: u64,
}
