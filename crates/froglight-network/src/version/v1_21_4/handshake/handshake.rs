#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, froglight_macros::FrogBuf)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct HandshakePacket {
    #[frog(var)]
    pub protocol: i32,
    pub address: String,
    pub port: u16,
    pub intent: ConnectionIntent,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, froglight_macros::FrogBuf)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, Clone, PartialEq, Hash))]
pub enum ConnectionIntent {
    /// The connection wants to get the status of the server.
    Status = 1,
    /// The connection wants to login to the server.
    #[default]
    Login = 2,
    /// The connection is being transferred from another server.
    Transfer = 3,
}
