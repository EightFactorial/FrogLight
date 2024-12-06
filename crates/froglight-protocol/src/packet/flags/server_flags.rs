use froglight_macros::FrogReadWrite;

/// The player's ability flags sent from the server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(bitset, tests = ["read_verify", "write_verify"], bytes = [0])]
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
