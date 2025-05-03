#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use smol_str::SmolStr;

use crate::common::ConnectionIntent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct HandshakePacket {
    #[frog(var)]
    pub protocol: i32,
    pub address: SmolStr,
    pub port: u16,
    pub intent: ConnectionIntent,
}
