#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

/// An action that a client can ask the server to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub enum ClientStatusAction {
    /// Respawn the player.
    PerformRespawn,
    /// Request the player's statistics.
    RequestStats,
}
