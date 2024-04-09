use froglight_macros::FrogReadWrite;

/// The player's ability flags sent from the server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(bitset, tests = ["read_verify", "write_verify"], bytes = [0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ServerPlayerAbilityFlags {
    /// Whether the player is invulnerable.
    pub invulnerable: bool,
    /// Whether the player is flying.
    pub flying: bool,
    /// Whether the player can fly.
    pub allow_flying: bool,
    /// Whether the player instant breaks blocks.
    pub instant_break: bool,
}
