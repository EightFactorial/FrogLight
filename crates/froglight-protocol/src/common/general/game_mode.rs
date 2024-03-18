use froglight_macros::FrogReadWrite;

/// The game mode of a player.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
pub enum GameMode {
    /// Survival mode.
    #[default]
    Survival,
    /// Creative mode.
    Creative,
    /// Adventure mode.
    Adventure,
    /// Spectator mode.
    Spectator,
}

macro_rules! impl_try_from {
    ($($t:ty),*) => {
        $(
            impl TryFrom<$t> for GameMode {
                type Error = ();
                fn try_from(intent: $t) -> Result<Self, Self::Error> {
                    match intent {
                        0 => Ok(GameMode::Survival),
                        1 => Ok(GameMode::Creative),
                        2 => Ok(GameMode::Adventure),
                        3 => Ok(GameMode::Spectator),
                        _ => Err(()),
                    }
                }
            }
            impl From<GameMode> for $t {
                fn from(gamemode: GameMode) -> Self {
                    match gamemode {
                        GameMode::Survival => 0,
                        GameMode::Creative => 1,
                        GameMode::Adventure => 2,
                        GameMode::Spectator => 3,
                    }
                }
            }
        )*
    };
}
impl_try_from!(i8, i16, i32, i64, i128, isize);
impl_try_from!(u8, u16, u32, u64, u128, usize);
