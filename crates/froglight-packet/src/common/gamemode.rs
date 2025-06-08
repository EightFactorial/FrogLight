#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
#[cfg(feature = "io")]
use froglight_io::prelude::*;
use serde::{Deserialize, Serialize};

/// The game mode of a player.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
#[cfg_attr(feature = "bevy", derive(Component, Reflect), require(PreviousGameMode))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash, Component))]
#[serde(rename_all = "lowercase")]
pub enum GameMode {
    /// Survival mode
    #[default]
    Survival,
    /// Creative mode
    Creative,
    /// Adventure mode
    Adventure,
    /// Spectator mode
    Spectator,
}

impl GameMode {
    /// Returns `true` if the game mode is [`GameMode::Survival`].
    #[inline]
    #[must_use]
    pub const fn is_survival(self) -> bool { matches!(self, GameMode::Survival) }

    /// Returns `true` if the game mode is [`GameMode::Creative`].
    #[inline]
    #[must_use]
    pub const fn is_creative(self) -> bool { matches!(self, GameMode::Creative) }

    /// Returns `true` if the game mode is [`GameMode::Adventure`].
    #[inline]
    #[must_use]
    pub const fn is_adventure(self) -> bool { matches!(self, GameMode::Adventure) }

    /// Returns `true` if the game mode is [`GameMode::Spectator`].
    #[inline]
    #[must_use]
    pub const fn is_spectator(self) -> bool { matches!(self, GameMode::Spectator) }

    /// Get the [`GameMode`] from its ID.
    #[must_use]
    pub const fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(GameMode::Survival),
            1 => Some(GameMode::Creative),
            2 => Some(GameMode::Adventure),
            3 => Some(GameMode::Spectator),
            _ => None,
        }
    }

    /// Get the ID of the [`GameMode`].
    #[must_use]
    pub const fn into_id(self) -> u8 {
        match self {
            GameMode::Survival => 0,
            GameMode::Creative => 1,
            GameMode::Adventure => 2,
            GameMode::Spectator => 3,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// The previous game mode of a player.
#[repr(transparent)]
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Deref,
    DerefMut,
    From,
    Into,
)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash, Component))]
#[serde(transparent)]
pub struct PreviousGameMode(pub Option<GameMode>);

impl PreviousGameMode {
    /// Create a new [`PreviousGameMode`] with the given [`GameMode`].
    #[inline]
    #[must_use]
    pub const fn new(mode: Option<GameMode>) -> Self { PreviousGameMode(mode) }

    /// Get the [`PreviousGameMode`] from its ID.
    #[must_use]
    pub const fn from_id(id: i8) -> Option<Self> {
        match id {
            -1 => Some(PreviousGameMode(None)),
            0 => Some(PreviousGameMode(Some(GameMode::Survival))),
            1 => Some(PreviousGameMode(Some(GameMode::Creative))),
            2 => Some(PreviousGameMode(Some(GameMode::Adventure))),
            3 => Some(PreviousGameMode(Some(GameMode::Spectator))),
            _ => None,
        }
    }

    /// Get the ID of the [`PreviousGameMode`].
    #[must_use]
    pub const fn into_id(self) -> i8 {
        match self {
            PreviousGameMode(None) => -1,
            PreviousGameMode(Some(GameMode::Survival)) => 0,
            PreviousGameMode(Some(GameMode::Creative)) => 1,
            PreviousGameMode(Some(GameMode::Adventure)) => 2,
            PreviousGameMode(Some(GameMode::Spectator)) => 3,
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl FrogRead for PreviousGameMode {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        match <i32 as FrogVarRead>::frog_var_read(buffer)? {
            -1 => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::read", "Reading enum variant \"PreviousGameMode::None\" (-1)");
                Ok(PreviousGameMode(None))
            }
            0 => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::read", "Reading enum variant \"PreviousGameMode::Survival\" (0)");
                Ok(PreviousGameMode(Some(GameMode::Survival)))
            }
            1 => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::read", "Reading enum variant \"PreviousGameMode::Creative\" (1)");
                Ok(PreviousGameMode(Some(GameMode::Creative)))
            }
            2 => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::read", "Reading enum variant \"PreviousGameMode::Adventure\" (2)");
                Ok(PreviousGameMode(Some(GameMode::Adventure)))
            }
            3 => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::read", "Reading enum variant \"PreviousGameMode::Spectator\" (3)");
                Ok(PreviousGameMode(Some(GameMode::Spectator)))
            }
            unk => Err(ReadError::InvalidEnum(core::any::type_name::<Self>(), unk)),
        }
    }
}

#[cfg(feature = "io")]
impl FrogWrite for PreviousGameMode {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        match self.0 {
            None => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"PreviousGameMode::None\" (-1)");
                <i32 as FrogVarWrite>::frog_var_write(&-1, buffer)
            }
            Some(GameMode::Survival) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"PreviousGameMode::Survival\" (0)");
                <i32 as FrogVarWrite>::frog_var_write(&0, buffer)
            }
            Some(GameMode::Creative) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"PreviousGameMode::Creative\" (1)");
                <i32 as FrogVarWrite>::frog_var_write(&1, buffer)
            }
            Some(GameMode::Adventure) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"PreviousGameMode::Adventure\" (2)");
                <i32 as FrogVarWrite>::frog_var_write(&2, buffer)
            }
            Some(GameMode::Spectator) => {
                #[cfg(feature = "trace")]
                tracing::trace!(target: "froglight_io::write", "Writing enum variant \"PreviousGameMode::Spectator\" (3)");
                <i32 as FrogVarWrite>::frog_var_write(&3, buffer)
            }
        }
    }

    #[inline]
    fn frog_len(&self) -> usize { i32::from(self.into_id()).frog_len() }
}
