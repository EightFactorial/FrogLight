#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

/// The intent a client has when connecting to a server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash, Component))]
pub enum ConnectionIntent {
    /// The client wants the status of the server.
    Status = 1,
    /// The client wants to login to the server.
    #[default]
    Login = 2,
    /// The client is being transferred from another server.
    Transfer = 3,
}
