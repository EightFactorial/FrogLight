#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_macros::FrogBuf;
use smol_str::SmolStr;

use crate::types::ConnectionIntent;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogBuf)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub struct HandshakePacket {
    #[frog(var)]
    pub protocol: i32,
    pub address: SmolStr,
    pub port: u16,
    pub intent: ConnectionIntent,
}
