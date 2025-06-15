#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "io")]
use froglight_io::prelude::*;
use serde::{Deserialize, Serialize};

/// The difficulty level of the game.
#[expect(missing_docs)]
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(
    feature = "bevy",
    reflect(Debug, Default, Clone, PartialEq, Hash, Resource, Serialize, Deserialize)
)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Peaceful,
    Easy,
    #[default]
    Normal,
    Hard,
}

impl Difficulty {
    /// Get the [`Difficulty`] as a [`u8`].
    #[must_use]
    pub const fn into_u8(self) -> u8 {
        match self {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        }
    }

    /// Get the [`Difficulty`] from a [`u8`].
    #[must_use]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Difficulty::Peaceful),
            1 => Some(Difficulty::Easy),
            2 => Some(Difficulty::Normal),
            3 => Some(Difficulty::Hard),
            _ => None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl FrogRead for Difficulty {
    fn frog_read(buffer: &mut impl std::io::Read) -> core::result::Result<Self, ReadError> {
        match u8::frog_read(buffer)? {
            val @ 0..3 => Ok(Difficulty::from_u8(val).unwrap_or_else(|| unreachable!())),
            invalid => Err(ReadError::InvalidEnum(core::any::type_name::<Self>(), invalid as i32)),
        }
    }
}

#[cfg(feature = "io")]
impl FrogWrite for Difficulty {
    #[inline]
    fn frog_write(
        &self,
        buffer: &mut impl std::io::Write,
    ) -> core::result::Result<usize, WriteError> {
        self.into_u8().frog_write(buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { self.into_u8().frog_len() }
}
