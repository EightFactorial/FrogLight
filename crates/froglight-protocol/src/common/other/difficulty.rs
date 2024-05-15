use std::fmt::Display;

#[cfg(feature = "bevy")]
use bevy_ecs::reflect::ReflectResource;
#[cfg(feature = "bevy")]
use bevy_reflect::std_traits::ReflectDefault;
use froglight_macros::FrogReadWrite;

/// The difficulty of the game.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [2])]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::system::Resource, bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Default, Resource))]
pub enum Difficulty {
    /// Peaceful difficulty.
    Peaceful,
    /// Easy difficulty.
    Easy,
    /// Normal difficulty.
    ///
    /// This is the default difficulty.
    #[default]
    Normal,
    /// Hard difficulty.
    Hard,
}

impl From<Difficulty> for u8 {
    fn from(difficulty: Difficulty) -> u8 {
        match difficulty {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        }
    }
}

impl TryFrom<u8> for Difficulty {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Difficulty::Peaceful),
            1 => Ok(Difficulty::Easy),
            2 => Ok(Difficulty::Normal),
            3 => Ok(Difficulty::Hard),
            _ => Err(()),
        }
    }
}

impl From<Difficulty> for i8 {
    fn from(difficulty: Difficulty) -> i8 { i8::try_from(u8::from(difficulty)).unwrap() }
}

impl TryFrom<i8> for Difficulty {
    type Error = ();
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Difficulty::try_from(u8::try_from(value).map_err(|_| ())?)
    }
}

impl From<Difficulty> for u32 {
    fn from(difficulty: Difficulty) -> u32 { u32::from(u8::from(difficulty)) }
}

impl TryFrom<u32> for Difficulty {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Difficulty::try_from(u8::try_from(value).map_err(|_| ())?)
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Peaceful => write!(f, "Peaceful"),
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Normal => write!(f, "Normal"),
            Difficulty::Hard => write!(f, "Hard"),
        }
    }
}
