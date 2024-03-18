use froglight_macros::FrogReadWrite;

/// A game state change event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum GameStateEvent {
    /// Invalid bed.
    NoRespawnBlockAvailable,
    /// The weather is set to rain.
    StartRaining,
    /// The weather is set to clear.
    StopRaining,
    /// The player's game mode is changed.
    ChangeGameMode,
    /// The player enters the credits screen.
    WinGame,
    /// Unknown cause.
    DemoEvent,
    /// An arrow hits a player.
    ArrowHitPlayer,
    /// The weather changed from thunder to rain.
    RainLevelChange,
    /// The weather changed from rain to thunder.
    ThunderLevelChange,
    /// The player is hurt by a pufferfish.
    PufferFishSting,
    /// The player is affected by an elder guardian.
    GuardianElderEffect,
    /// The player immediately respawns.
    ImmediateRespawn,
}
