#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
#[cfg(feature = "io")]
use froglight_io::prelude::*;
use serde::{Deserialize, Serialize};

/// The game mode of a player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
#[serde(rename_all = "lowercase")]
pub enum GameMode {
    /// Survival mode
    Survival,
    /// Creative mode
    Creative,
    /// Adventure mode
    Adventure,
    /// Spectator mode
    Spectator,
}

// -------------------------------------------------------------------------------------------------

/// The previous game mode of a player.
#[repr(transparent)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Deref, DerefMut, From, Into,
)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
#[serde(transparent)]
pub struct PreviousGameMode(Option<GameMode>);

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl FrogRead for PreviousGameMode {
    fn frog_read(_buffer: &mut impl std::io::Read) -> Result<Self, ReadError> { todo!() }
}

#[cfg(feature = "io")]
impl FrogWrite for PreviousGameMode {
    fn frog_write(&self, _buffer: &mut impl std::io::Write) -> Result<usize, WriteError> { todo!() }

    fn frog_len(&self) -> usize { todo!() }
}
