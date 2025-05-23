#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

/// The intent a client has when connecting to a server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
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
