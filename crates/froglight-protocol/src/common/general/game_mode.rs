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
